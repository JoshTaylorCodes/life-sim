// Leaves the body with the `Leave` button and turns the wheel without moving the pointer. The
// `Guide` button slides under the pointer when `Leave` goes, and the wheel must still zoom within
// 500 ms. Throws when it does not. Use with `strip --play`.
export default async function (page) {
  const scale = () => page.evaluate(() => window.__tidepool.camera.scale);
  await page.getByRole('button', { name: /Leave/ }).click();
  await page.waitForFunction(() => window.__tidepool.mode === 'watch', null, { timeout: 5000 });
  const before = await scale();
  await page.mouse.wheel(0, -400);
  const zoomed = await page
    .waitForFunction((s) => window.__tidepool.camera.scale !== s, before, { timeout: 500 })
    .then(() => true, () => false);
  if (!zoomed) {
    const under = await page.evaluate(() => {
      const el = document.elementFromPoint(innerWidth - 60, 34);
      return el ? `${el.tagName.toLowerCase()}.${el.className}` : 'nothing';
    });
    throw new Error(`the wheel after Leave left the scale at ${before} for 500 ms (the pointer is over ${under})`);
  }
  console.log(`leave-wheel: scale ${before} -> ${await scale()}`);
}
