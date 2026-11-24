// Opens the field guide with its key and goes to chapter 2 with Next. Use with `handoff` or `strip`.
export default async function (page) {
  await page.keyboard.press('KeyG');
  const guide = page.getByRole('dialog', { name: 'Field guide' });
  await guide.waitFor({ timeout: 5000 });
  await guide.getByRole('button', { name: /Next/ }).click();
  await guide.getByText('Chapter 2 of', { exact: false }).waitFor({ timeout: 5000 });
}
