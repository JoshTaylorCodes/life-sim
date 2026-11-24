// Waits for the in-page bench of `?bench` to answer, and checks its line. Use with `handoff`.
export default async function (page) {
  const line = page.locator('p.bench');
  // `handoff` waits 3 s before the act: "Measuring." may be gone by then (features/bench.md).
  if (!(await line.isVisible())) throw new Error('bench: no p.bench line. Fix: open the page with ?bench.');
  await line.filter({ hasText: 'ticks per second at' }).waitFor({ timeout: 240000 });
  const text = await line.textContent();
  if (!/^[\d,]+ ticks per second at [\d,]+ creatures\. A 24,000-tick skip would take [\d.]+ seconds on this device\./.test(text)) {
    throw new Error(`bench: the line does not read as a result: ${text}`);
  }
  console.log(`bench: ${text}`);
}
