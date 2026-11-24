# Walkthrough

Twelve beats (`BEATS` in `src/app/walkthrough.ts`, plan `tidepool/docs/plans/2026-10-13-walkthrough.md`). Each names what the player just did in one line at the foot of the screen, a pulse on a button, or the death screen, and never blocks play. The beats done and shown are kept in `localStorage` (`tidepool.walk`).

## Sub-features

- `walk-lines`: the foot line (`role=status`), one at a time: "Click any creature to become it." in Watch, "Your instincts are steering. Move to take over." after 4 s with no input in Play, then eat, let go, half ring, score, court.
- `walk-pulse`: the `Brain` button pulses for beat 5, the counter for beat 9.
- `walk-store`: a reload keeps the beats done. Blocked storage keeps them in memory for the visit.
- `walk-reset`: `?walk=reset` clears the store first.
- `walk-log`: `?debug` shows "Copy beat times".

## How to get to it (user POV)

- Open the page for the first time, or with `?walk=reset`.

## Driving it with tidepool-verify

- **First beats, store, reset.** `strip "<url>?walk=reset" $E/walk --play --frames 2 --act .claude/skills/verify-tidepool/acts/walk.mjs`. The act checks that the pick finished `possess`, waits for the line of beat 2, holds W for 100 ticks and checks `move`, reloads without the query and checks both beats are kept, then reloads with `?walk=reset` and checks `walk.done` is empty and the line of beat 1 is up.
- **Later beats.** Read `window.__tidepool.walk` (`done`, `shown`, `now`) in an act. Beats 3 to 12 need eating, a bud, a mate, or a death: drive them with `play.md` and `death.md`.

## Gotchas

- The line of beat 1 shows in Watch to every visitor, so it does not prove a reset. `walk.done` does.
- A beat waits `GAP_S` (4 s) after a foot line, and an ignored beat retires after 2 showings.
- The walk's clock takes at most 0.1 s from one drawn frame (`WALK_DT_MAX`). In the software GPU the page draws a few frames per second, so a 4 s beat takes longer than 4 s of wall time. Wait on the line, not on a timer.
- Beat 2 needs 3 body lengths under the player's input. With W held for 40 ticks the act saw `move` not done (2026-11-23). It holds W for 100 ticks.
