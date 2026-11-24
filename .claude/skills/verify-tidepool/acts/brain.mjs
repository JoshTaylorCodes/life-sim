// Opens the brain panel of the body in Play with its key, and checks that it shows. Use with `strip --play`.
export default async function (page, { step }) {
  await page.keyboard.press('KeyB');
  await page.getByRole('region', { name: 'Brain' }).waitFor({ timeout: 5000 });
  await step(page, 2);
}
