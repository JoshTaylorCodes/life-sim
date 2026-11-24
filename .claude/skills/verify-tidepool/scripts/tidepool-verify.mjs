#!/usr/bin/env node
// Drives the Tidepool dev page in its own headless Chromium. Commands and flags: SKILL.md.
import { createRequire } from 'node:module';
import { execSync } from 'node:child_process';
import { copyFileSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const TIDEPOOL = resolve(dirname(fileURLToPath(import.meta.url)), '../../../../tidepool');
const die = (msg) => {
  console.error(`error: ${msg}`);
  process.exit(1);
};

const [cmd, ...rest] = process.argv.slice(2);
const flags = {};
const args = [];
for (let i = 0; i < rest.length; i++) {
  if (rest[i].startsWith('--')) {
    const next = rest[i + 1];
    flags[rest[i].slice(2)] = next === undefined || next.startsWith('--') ? true : rest[++i];
  } else args.push(rest[i]);
}
const [vw, vh] = String(flags.size ?? '1300x700').split('x').map(Number);

function playwright() {
  try {
    const root = execSync('npm root -g').toString().trim();
    return createRequire(join(root, 'noop.js'))('playwright');
  } catch {
    die('Playwright is not installed globally. Fix: npm i -g playwright && npx playwright install chromium');
  }
}

// --- doctor: read-only. Is this server worth driving?
async function doctor(url) {
  const page = await fetch(url).catch((e) => die(`${url} does not answer (${e.message}). Fix: start the dev server (SKILL.md, Launch).`));
  const html = await page.text();
  if (!html.includes('<title>Tidepool</title>')) die(`${url} serves a page that is not Tidepool. Fix: another server owns this port, pick a free port.`);
  const vite = await fetch(new URL('@vite/client', url));
  if (!vite.ok) die(`${url} is not the Vite dev server (no /@vite/client). window.__tidepool exists only in dev. Fix: run the dev server, not vite preview or dist/.`);
  const want = Number(/FORMAT_VERSION: u32 = (\d+)/.exec(readFileSync(join(TIDEPOOL, 'crates/sim/src/snapshot.rs'), 'utf8'))?.[1]);
  const world = new Uint8Array(await (await fetch(new URL('m2-world.tdpl', url))).arrayBuffer());
  const magic = String.fromCharCode(...world.slice(0, 4));
  const got = world[4] | (world[5] << 8) | (world[6] << 16) | (world[7] << 24);
  if (magic !== 'TDPL') die('public/m2-world.tdpl is missing. Fix: make headless && target/release/tidepool-headless run --seed 1 --ticks 36000 --creatures 400 --plants 1200 --out public/m2-world.tdpl');
  if (got !== want) die(`public/m2-world.tdpl is format ${got}, the core reads ${want}: the page will fail with BadVersion. Fix: rebake it (same command as a missing world).`);
  const port = new URL(url).port;
  let owner = '';
  try {
    owner = execSync(`lsof -nP -iTCP:${port} -sTCP:LISTEN`).toString().trim().split('\n').slice(1).join('; ');
  } catch {
    owner = '(lsof found no listener)';
  }
  console.log(`ok: Tidepool dev server at ${url}, baked world format ${got}, port ${port} owner: ${owner}`);
}

// --- the browser
async function open(url) {
  const { chromium } = playwright();
  const browser = await chromium.launch({ args: ['--use-angle=swiftshader', '--enable-unsafe-swiftshader'] });
  const page = await browser.newPage({ viewport: { width: vw, height: vh } });
  const problems = [];
  page.on('console', (m) => {
    // The screenshot itself reads pixels back from the software GPU. That stall note is the harness's.
    if (m.text().includes('GPU stall due to ReadPixels')) return;
    if (m.type() === 'error' || m.type() === 'warning') problems.push(`console.${m.type()}: ${m.text()}`);
  });
  page.on('pageerror', (e) => problems.push(`pageerror: ${e.message}`));
  page.on('requestfailed', (r) => problems.push(`requestfailed: ${r.url()} ${r.failure()?.errorText}`));
  await page.goto(url);
  const ready = await page
    .waitForFunction(() => document.querySelector('#ui > .panel[role=alert]') || (window.__tidepool?.frame && !document.querySelector('#ui > .panel')), null, { timeout: 60000 })
    .then(() => true, () => false);
  const alert = await page.locator('#ui > .panel[role=alert]').textContent({ timeout: 100 }).catch(() => null);
  return { browser, page, problems, ready, alert };
}

const tick = (page) => page.evaluate(() => window.__tidepool.frame.tick);
const settle = async (page) => {
  for (let last = await tick(page); ; ) {
    await page.waitForTimeout(250);
    const now = await tick(page);
    if (now === last) return now;
    last = now;
  }
};
// The clock: `__tidepool.setSpeed` (dev only, src/app/main.ts) sends the worker its `speed`.
async function freeze(page) {
  await page.evaluate(() => window.__tidepool.setSpeed(0));
  const t = await settle(page);
  await page.waitForTimeout(500);
  if ((await tick(page)) !== t) die('the world did not stop at speed 0. Fix: check setSpeed in src/app/main.ts.');
  return t;
}
// Runs the world to exactly `target`, at half the ticks left per second (20 at most, 1 at least). A
// software-GPU draw can hold the main thread, and the page's frames, for most of a second: the world
// must not reach the target in that time.
async function stepTo(page, target) {
  const sent = await page.evaluate(
    (target) =>
      new Promise((done) => {
        const tp = window.__tidepool;
        const log = [];
        let speed = -1;
        const loop = () => {
          const t = tp.frame.tick;
          const want = t >= target ? 0 : Math.min(20, Math.max(1, Math.floor((target - t) / 2)));
          if (want !== speed) {
            tp.setSpeed((speed = want));
            log.push(`tick ${t} speed ${want}`);
          }
          if (want === 0) return done(log);
          setTimeout(loop, 10);
        };
        loop();
      }),
    target,
  );
  const t = await settle(page);
  if (t !== target) die(`the world stopped at tick ${t}, not ${target} (sent: ${sent.join(', ')}). Fix: slow the last ticks in stepTo().`);
  return t;
}
const step = async (page, ticks) => stepTo(page, (await tick(page)) + ticks);

// The fittest body in view that a click will take (input.ts `clickOn`), as a screen point.
async function pickTarget(page) {
  return page.evaluate(async () => {
    const P = await import('/src/protocol.ts');
    const { frame: f, camera: c } = window.__tidepool;
    const W = innerWidth;
    const H = innerHeight;
    let best = null;
    for (let i = 0; i < f.creatureCount; i++) {
      const r = f.creatureBase + i * P.CREATURE_STRIDE;
      const e = f.f32[r + P.C_ENERGY];
      const sx = (f.f32[r + P.C_X] - c.x) * c.scale + W / 2;
      const sy = (f.f32[r + P.C_Y] - c.y) * c.scale + H / 2;
      const inside = sx > 80 && sx < W - 80 && sy > 80 && sy < H - 120;
      if (inside && f.f32[r + P.C_AGE] < 0.5 && (!best || e > best.e)) best = { e, sx, sy, uid: f.u32[r + P.C_UID] };
    }
    return best;
  });
}

// `--act <file.mjs>`: its default export runs on the ready page, before the shots. It gets the page
// and the clock: `async (page, { freeze, stepTo, step }) => {}`.
async function act(page) {
  const mod = await import(resolve(flags.act));
  await mod.default(page, { freeze, stepTo, step });
}

async function shoot(page, dir, name) {
  await page.evaluate(() => new Promise((r) => requestAnimationFrame(() => requestAnimationFrame(r))));
  const path = join(dir, name);
  await page.screenshot({ path });
  return path;
}

// A grid of labelled images, drawn by the same browser.
async function montage(browser, cells, out, cols) {
  const page = await browser.newPage({ viewport: { width: 1600, height: 400 } });
  const imgs = cells
    .map((c) => `<figure><img src="data:image/png;base64,${readFileSync(c.path).toString('base64')}"><figcaption>${c.label}</figcaption></figure>`)
    .join('');
  await page.setContent(
    `<style>body{margin:0;background:#111;color:#eee;font:14px system-ui}main{display:grid;grid-template-columns:repeat(${cols},1fr);gap:6px;padding:6px}figure{margin:0}img{width:100%;display:block}</style><main>${imgs}</main>`,
  );
  await page.screenshot({ path: out, fullPage: true });
  await page.close();
}

async function finish(browser, dir, report) {
  writeFileSync(join(dir, `${cmd}.json`), JSON.stringify(report, null, 2));
  await browser.close();
  const bad = report.problems.length > 0 || report.alert || !report.ready;
  console.log(`${bad ? 'FAIL' : 'ok'}: ${cmd} → ${dir}/${cmd}.json`);
  if (report.problems.length) console.log(report.problems.join('\n'));
  if (report.alert) console.log(`error panel: ${report.alert}`);
  process.exit(bad ? 1 : 0);
}

const [url, dirArg] = args;
const needDir = () => {
  if (!url || !dirArg) die(`usage: tidepool-verify.mjs ${cmd} <url> <evidence dir> [flags]`);
  mkdirSync(dirArg, { recursive: true });
  return resolve(dirArg);
};

if (cmd === 'doctor') {
  if (!url) die('usage: tidepool-verify.mjs doctor <url>');
  await doctor(url);
} else if (cmd === 'handoff') {
  const dir = needDir();
  const { browser, page, problems, ready, alert } = await open(url);
  if (ready) await page.waitForTimeout(3000);
  if (ready && flags.act) await act(page);
  const shot = await shoot(page, dir, 'handoff.png');
  await finish(browser, dir, { url, size: `${vw}x${vh}`, ready, alert, problems, shot, tick: ready && !alert ? await tick(page) : null });
} else if (cmd === 'strip') {
  const dir = needDir();
  const frames = Number(flags.frames ?? 8);
  const every = Number(flags.every ?? 10);
  const { browser, page, problems, ready, alert } = await open(url);
  if (!ready || alert) await finish(browser, dir, { url, ready, alert, problems });
  // Every run starts its strip at the same tick: the baked world loads at 36,000, `?founders` at 0.
  const stopped = await freeze(page);
  const from = Number(flags.from ?? (stopped >= 36000 ? 36040 : 40));
  if (stopped > from) die(`the world ran to tick ${stopped} before it stopped, past --from ${from}. Fix: pass a later --from.`);
  if (stopped < from) await stepTo(page, from);
  let possessed = null;
  if (flags.play) {
    const target = await pickTarget(page);
    if (!target) die('no body in view to take. Fix: pass a larger --size or drop --play.');
    await page.mouse.click(target.sx, target.sy);
    const took = await page
      .waitForFunction(() => window.__tidepool.mode === 'play', null, { timeout: 5000 })
      .then(() => true, () => false);
    if (!took) die(`a click at ${Math.round(target.sx)},${Math.round(target.sy)} on uid ${target.uid} did not take the body`);
    possessed = await page.evaluate(() => window.__tidepool.possessedUid);
    await step(page, every); // the camera moves to the body
  }
  if (flags.act) await act(page);
  const shots = [];
  for (let i = 0; i < frames; i++) {
    const t = i === 0 ? await tick(page) : await step(page, every);
    const cam = await page.evaluate(() => window.__tidepool.camera);
    shots.push({ i, tick: t, camera: cam, path: await shoot(page, dir, `frame-${String(i).padStart(2, '0')}.png`), label: `#${i} tick ${t}` });
  }
  await montage(browser, shots, join(dir, 'strip.png'), 4);
  await finish(browser, dir, { url, size: `${vw}x${vh}`, ready, alert, problems, mode: flags.play ? 'play' : 'watch', possessed, every, shots, strip: join(dir, 'strip.png') });
} else if (cmd === 'side') {
  const [ref, shot, dirS] = args;
  if (!ref || !shot || !dirS) die('usage: tidepool-verify.mjs side <reference.png> <capture.png> <evidence dir>');
  mkdirSync(dirS, { recursive: true });
  copyFileSync(ref, join(dirS, `reference-${basename(ref)}`));
  const { chromium } = playwright();
  const browser = await chromium.launch();
  await montage(browser, [{ path: ref, label: `reference: ${ref}` }, { path: shot, label: `capture: ${shot}` }], join(dirS, 'side.png'), 2);
  await browser.close();
  console.log(`ok: side → ${join(resolve(dirS), 'side.png')}`);
} else {
  die('usage: tidepool-verify.mjs <doctor|handoff|strip|side> ... (see .claude/skills/verify-tidepool/SKILL.md)');
}
