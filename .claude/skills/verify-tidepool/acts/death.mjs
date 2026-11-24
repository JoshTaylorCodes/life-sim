// The death overlay, both ways out. Use with `strip --play`. `move_cost` at its maximum, with W held,
// starves the player's body. It goes back to its default before each way out. On 2026-11-24 this left
// 1 heir. `basal` and `fixed_upkeep` at their maximum left none: the brood starved first.
// The first death: pick the first heir, and the page is in Play in that heir. The second: `Keep
// watching`, and the page is in Watch. With no heir the first death takes `Keep watching` too.
// The act lets the body bud first (up to 120 s) so that it has an heir.
import { readFileSync } from 'node:fs';

const params = readFileSync(new URL('../../../../tidepool/crates/sim/src/params.rs', import.meta.url), 'utf8');
const STARVE = ['move_cost'].map((name) => {
  const [, base, , max] = new RegExp(`"${name}" ([\\d.]+) ([\\d.]+) ([\\d.]+);`).exec(params).map(Number);
  return { name, base, max };
});
const set = (page, key) => page.evaluate((all) => all.forEach((p) => window.__tidepool.setParam(p.name, p.value)), STARVE.map((p) => ({ name: p.name, value: p[key] })));

// Not `step`: a death pauses the world, and `stepTo` sends no new speed while its wanted speed holds,
// so it waits for a tick that never comes. The world runs at full speed until the overlay shows.
export default async function (page) {
  const overlay = page.getByRole('region', { name: 'You died' });
  const die = async () => {
    await set(page, 'max');
    await page.evaluate(() => window.__tidepool.setSpeed(20));
    await page.keyboard.down('KeyW');
    const shown = await overlay.waitFor({ timeout: 180000 }).then(() => true, () => false);
    await page.keyboard.up('KeyW');
    if (!shown) throw new Error(`death: no overlay after 180 s of starving at ${JSON.stringify(STARVE)}`);
    await set(page, 'base');
    return page.evaluate(() => window.__tidepool.death);
  };
  // An heir is a living descendant: let the body bud first (the counter counts the line, the player apart).
  await page.evaluate(() => window.__tidepool.setSpeed(20));
  await page.waitForFunction(() => window.__tidepool.frame.lineageAlive > 1, null, { timeout: 120000 }).catch(() => {});
  const first = await die();
  const heir = overlay.locator('button.heir').first();
  let picked = null;
  if (await heir.count()) {
    const was = await page.evaluate(() => window.__tidepool.possessedUid);
    await heir.click();
    await page.waitForFunction((w) => window.__tidepool.mode === 'play' && window.__tidepool.possessedUid !== w, was, { timeout: 5000 });
    picked = await page.evaluate(() => window.__tidepool.possessedUid);
    await die();
  }
  await overlay.getByRole('button', { name: 'Keep watching' }).click();
  await page.waitForFunction(() => window.__tidepool.mode === 'watch' && window.__tidepool.death === null, null, { timeout: 5000 });
  console.log(`death: cause ${first.cause}, ${first.heirs.length} heirs, picked ${picked ?? 'none'}, then Keep watching`);
}
