// The first beats of the walkthrough, on a page opened with `?walk=reset`. Use with `strip --play`.
// Beat 1 is done by the pick, beat 2's line comes up with no input, and W finishes beat 2. A reload
// keeps the beats, and a reload with `?walk=reset` clears them. Throws at the first step that fails.
const MOVE = 'Your instincts are steering. Move to take over.';

export default async function (page, { freeze, step }) {
  const done = () => page.evaluate(() => window.__tidepool.walk.done);
  const need = (ok, what) => {
    if (!ok) throw new Error(`walk: ${what}`);
  };
  need((await done()).includes('possess'), 'the pick did not finish beat 1 (possess)');
  const hint = page.getByRole('status').filter({ hasText: MOVE });
  // The walk's clock takes at most 0.1 s from one drawn frame (`WALK_DT_MAX`): a slow software GPU slows it.
  const shown = await hint.waitFor({ timeout: 30000 }).then(() => true, () => false);
  need(shown, `no line "${MOVE}" after 30 s with no input: walk ${JSON.stringify(await page.evaluate(() => window.__tidepool.walk))}`);
  await page.keyboard.down('KeyW');
  await step(page, 100);
  await page.keyboard.up('KeyW');
  need((await done()).includes('move'), `five seconds of W did not finish beat 2 (move): body ${JSON.stringify(await page.evaluate(() => [window.__tidepool.body, window.__tidepool.brainDriving]))}`);
  const url = new URL(page.url());
  url.searchParams.delete('walk');
  const ready = () => page.waitForFunction(() => window.__tidepool?.frame, null, { timeout: 60000 });
  await page.goto(url.href);
  await ready();
  const kept = await done();
  need(kept.includes('possess') && kept.includes('move'), `a reload lost the beats: ${JSON.stringify(kept)}`);
  url.searchParams.set('walk', 'reset');
  await page.goto(url.href);
  await ready();
  need((await done()).length === 0, `?walk=reset left ${JSON.stringify(await done())}`);
  await page.getByRole('status').filter({ hasText: 'Click any creature to become it.' }).waitFor({ timeout: 5000 });
  await freeze(page);
  console.log(`walk: beats ${JSON.stringify(kept)} kept on reload, cleared by ?walk=reset`);
}
