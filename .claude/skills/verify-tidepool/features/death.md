# Death overlay

When the possessed body dies, the worker pauses the world and the page shows the region `You died`: the cause ("You starved.", "Eaten.", ...), a killer card with the killer's body and brain when a body killed it, the heirs as buttons ("Generation n", an energy bar), and `Keep watching`. With no heir it says "Your line has ended." The first death of a life of 20 seconds or more that leaves an heir leads with "You are gone. Your line is not." and the cause comes after the heirs.

## Sub-features

- `death-cause`: the cause line, and the `?` note on "Eaten.".
- `death-killer`: the killer card, or "Its brain could not be read.".
- `death-heirs`: a pointer or the focus on an heir moves the camera to it. A click possesses it. An heir that died first gives "That one is gone.".
- `death-watch`: `Keep watching` goes back to Watch and ends the pause.

## How to get to it (user POV)

- Play a body until it starves, is eaten, poisoned, or dies of old age.

## Driving it with tidepool-verify

- **Both ways out.** `strip <url> $E/death --play --frames 4 --act .claude/skills/verify-tidepool/acts/death.mjs`. The act runs the world until the body buds (up to 120 s), then starves it with `move_cost` at its maximum and W held, and resets `move_cost` when the overlay shows. It picks the first heir (`mode === 'play'` in a new uid), starves that one, then chooses `Keep watching` (`mode === 'watch'`, `death === null`). It prints the cause, the heir count, and the uid it picked, and throws at the first step that fails. A line with "0 heirs, picked none" drove `Keep watching` only. About 35 s.
- **Phone.** Add `--size 390x844`, and take the shots before the way out (an act that stops at the overlay). The overlay must not cover the possessed body's place or the top buttons.

## Gotchas

- `basal` or `fixed_upkeep` at the maximum starves the brood before the parent: the overlay showed 0 heirs twice. `move_cost` with W held left 1 heir. An heir that dies before the pick gives "That one is gone.": the error state working, not a defect.
- Do not run the world to a death with `step` or `stepTo`. A death pauses the world, and `stepTo` sends no new speed while its wanted speed holds, so it waits forever (a run hung 11 hours on 2026-11-23). Use `setSpeed(20)` and wait on the region `You died`.
- The overlay owns the keyboard. Steering keys do nothing while it is up. Escape closes an open note first.
- A `speed` message ends the pause of a death, so `step` behind the overlay runs the world.
