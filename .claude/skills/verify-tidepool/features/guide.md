# Field guide

A modal dialog over the running world: chapters 1 to 6 ("A body", "A brain", "A child", "A line", ...), a glossary, and "In your world, now" boxes with live numbers. At 44rem and under it is a sheet with the chapters in one row.

## Sub-features

- `guide-open`: the `Guide` button or G opens it, G, Escape, or the close button closes it.
- `guide-nav`: the chapter list, `Next` and `Back`.
- `guide-live`: the live boxes read the player's world.
- `guide-notes`: a `?` mark opens a note in place, Escape closes the note first.

## How to get to it (user POV)

- Choose `Guide` or press G, in Watch or in Play.

## Driving it with tidepool-verify

- **Open and page.** `handoff <url> $E/guide --act .claude/skills/verify-tidepool/acts/guide.mjs`. The dialog named `Field guide` shows "Chapter 2 of 6" and the console is clean.
- **Phone.** Add `--size 390x844`. The sheet covers the foot of the screen and the top buttons stay visible.

## Gotchas

- While the guide is open it owns the keyboard. Steering keys do nothing until it closes.
- When the possessed body dies, the guide closes and the death overlay takes the focus (`2c022ac0`).
