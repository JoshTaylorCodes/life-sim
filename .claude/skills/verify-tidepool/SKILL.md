---
name: verify-tidepool
description: Drive the Tidepool browser game (tidepool/, WebGL + WASM worker, Preact UI) in a private headless Chromium and capture evidence. Frame strips at exact sim ticks for motion and rendering, a hand-off check of the exact URL, and reference side-by-sides. Use for the Prove step of any Tidepool change the player can see or feel, before the hand-off. Not for the draft in /src.
---

# Verify Tidepool

Tidepool is the active project in this repo. The surface is one web page: a canvas of creatures (`#view`) under a Preact overlay (`#ui`). The sim runs in a worker. The dev server exposes a test handle, `window.__tidepool` (`tidepool/src/app/main.ts`, dev builds only). The helper drives the page through that handle and the real input path: clicks on the canvas, keys, and buttons.

The draft in `/src` (repo root `index.html`) is out of scope. It has no handle and no map here.

Helper: `.claude/skills/verify-tidepool/scripts/tidepool-verify.mjs`. Run it from the repo root with `node`. It uses the global Playwright (`npm root -g`). If it is missing, run `npm i -g playwright && npx playwright install chromium`. Each run starts its own headless Chromium. Never use the shared browser MCP tools (Playwright MCP, chrome-devtools, claude-in-chrome): other agents use them.

## Launch

Pick a free port first. A reused port can serve a stale module.

```sh
lsof -i :5391 || echo free          # try 5391, 5392, ... until free
cd tidepool && . "$HOME/.cargo/env" && make wasm
pnpm exec vite --port 5391 --strictPort --host 127.0.0.1   # run in the background, note its PID
```

`make dev` builds the WASM and starts Vite on its default port, which can collide with other sessions. Use `make wasm` and the line above instead. Ready means Vite prints `Local: http://127.0.0.1:5391/`.

The page loads `public/m2-world.tdpl`, which is not in git. A world from an older core fails with `BadVersion`. The doctor checks this.

Only one server per port. Two instances on two ports are fine. Two long headless sims at once die silently (`tidepool/AGENTS.md`), but the page is not a long sim.

## Doctor

Read-only. Run it first, and again whenever anything looks off.

```sh
node .claude/skills/verify-tidepool/scripts/tidepool-verify.mjs doctor http://127.0.0.1:5391/
```

It checks: the URL answers with the Tidepool page, it is the Vite dev server (the handle exists only there), the baked world's format equals `FORMAT_VERSION` in `crates/sim/src/snapshot.rs`, and which PID listens on the port. Compare that PID with the one you started. Each failure prints its fix.

## Drive

All commands take `<url> <evidence dir>` and these flags:

| Flag | Default | Does |
|---|---|---|
| `--size WxH` | `1300x700` | Viewport. Use `390x844` for the phone layout. |
| `--act <file.mjs>` | none | Runs `export default async (page, { freeze, stepTo, step }) => {}` on the ready page, before the shots. Ready-made acts are in `acts/`. |
| `--play` | off | `strip` only. Clicks the fittest young body in view, as a player does, and waits for Play mode. |
| `--frames N` | `8` | `strip` only. |
| `--every T` | `10` | `strip` only. Sim ticks between frames (20 ticks is one second). |
| `--from T` | `36040` | `strip` only. The tick of the first frame (`40` on a `?founders` world). |

```sh
S=.claude/skills/verify-tidepool/scripts/tidepool-verify.mjs
E=tidepool/verify-evidence/$(date +%F)-<slug>
node $S handoff http://127.0.0.1:5391/ $E/handoff
node $S strip   http://127.0.0.1:5391/ $E/watch
node $S strip   http://127.0.0.1:5391/ $E/play --play
node $S strip   http://127.0.0.1:5391/ $E/brain-phone --play --size 390x844 --act .claude/skills/verify-tidepool/acts/brain.mjs
node $S side    <reference.png> $E/play/frame-03.png $E/side
```

The clock: `__tidepool.setSpeed(0)` holds the world still. The strip freezes the world, then steps it to exact ticks: half the ticks left per second, so a slow software-GPU draw cannot make it overshoot. The baked world, the same ticks, and no input give the same bodies in the same places on every run. The same uid is picked on every run. Flagella, cilia, and the bud pulse animate on wall time, so two runs differ in about 1% of pixels. Compare positions and shapes, not bytes.

Handles that are stable: the buttons by role and name (`Brain`, `Guide`, `Leave`), the keys (`KeyW`/`KeyA`/`KeyD` or arrows steer, `Space` dash, `KeyC` court, `KeyQ` toxin, `Escape` leave, `KeyB` brain, `KeyG` guide), the region `Brain`, the dialog labelled by `#guide-title`, the death overlay `aria-label="You died"`, and the error panel `#ui > .panel[role=alert]`. In an act, read state from `window.__tidepool`: `mode`, `possessedUid`, `body`, `frame`, `camera`, `death`, `walk`, `stats`. `setParam(name, value)` changes a sim parameter and lists the names when the name is wrong.

Per-feature recipes are in `features/README.md`. A proof that drives one entry point is incomplete when the feature file lists others.

## Evidence

Put every run under `tidepool/verify-evidence/<date>-<slug>/` (gitignored). Each command writes its PNGs and a JSON report (`handoff.json`, `strip.json`) with the URL, viewport, ticks, camera, possessed uid, and every console problem.

- Drive the real user path: a click on the canvas, a key, a button. Use `setParam` and `setSpeed` only to set up a scene or hold the clock, never to fake the result you claim.
- Capture the action and the state after it, not only the last screen.
- Motion, rendering, physics, camera, or body shape: a frame strip (`strip.png`, 6 to 12 frames), never one screenshot. Open the strip and each frame that matters. In each frame, look for:
  - clipping: a part or body cut at the screen edge, at a panel edge, or at its own outline,
  - z-fighting and draw order: a body that flickers over and under another between frames, a panel drawn under the canvas,
  - jitter: the possessed body or the camera jumps between equal steps,
  - wrong relative sizes: the possessed body against its neighbours, a small body that lost its parts (flagella, cilia), a part not dimmed with its body,
  - overlap of UI: the hint line, the bud or mate card, the counter, the brain sheet, the guide, and the death overlay. Check `390x844` as well as `1300x700`.
- Reference named in the brief: run `side <reference> <capture>`. It copies the reference into the evidence folder and writes `side.png`. List each visible difference. "Matches" is a claim that `side.png` must show.
- A pixel-exact spec: capture a baseline before the change and never edit it.

## Hand-off check

Before you give the owner a link, open that exact link:

```sh
node .claude/skills/verify-tidepool/scripts/tidepool-verify.mjs handoff <exact URL you will give> $E/handoff
```

It exits 0 only when the page reached the world (no loading panel, no error panel) and the console holds no error or warning, no page error, and no failed request. The one note it ignores is Chromium's "GPU stall due to ReadPixels", which the screenshot itself causes. Use the same host and port as the link. If the owner plays on a phone, add `--size 390x844`. A link that was not opened is not a hand-off.

## Cleanup

Stop only the server you started: `kill <PID from Launch>`. Never kill by name: other sessions run Vite and Chromium here. The helper closes its own browser on exit. Cleanup never touches `tidepool/verify-evidence/`.

## Gotchas

- `vite preview` and `dist/` have no `__tidepool`. The doctor rejects them.
- A hidden tab pauses the world (`visibilitychange`). Headless pages stay visible.
- A click on a starving or old body is refused ("That one is starving"). `--play` picks a young body with the most energy.
- The worker drops frames when the page holds every buffer. Read the tick from `__tidepool.frame.tick` after a freeze, not while the world runs.
- A strip takes about 50 seconds on a loaded machine. Its stepping is exact: if a step overshoots, the helper stops with the ticks it saw.
