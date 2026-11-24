# Play a body

A click on a body in Watch possesses it. The camera follows it at `PLAY_SCALE`. Its brain drives until the player steers, and a key or a held pointer takes over. Cards come up beside the body: the bud card after a bud, the mate card next to a willing body.

## Sub-features

- `play-take`: a click on a young body with energy takes it. A starving or old body is refused with a line.
- `play-steer`: W A D or the arrows, or a held pointer. The hint "Your instincts are steering. Move to take over." goes away.
- `play-abilities`: `Dash` (Space) and the toxin (Q) when the body has the part. `Court` (C) when mixing is on.
- `play-leave`: `Leave` or Escape goes back to Watch.
- `play-cards`: the counter "Your line: n alive", the bud card, the mate card.

## How to get to it (user POV)

- In Watch, click any creature.
- On the death overlay, pick an heir.

## Driving it with tidepool-verify

- **Take and follow.** `strip <url> $E/play --play`. `strip.json` has `mode: "play"`, a `possessed` uid, and `camera.scale` 4 on each frame. The yellow-ringed body stays near the centre in every frame.
- **Steer.** An act that holds a key across steps:
  ```js
  export default async (page, { step }) => {
    await page.keyboard.down('KeyA');
    await step(page, 20);
    await page.keyboard.up('KeyA');
  };
  ```
  `window.__tidepool.brainDriving` is false after the press, and the body turned left between the frames.
- **Leave.** `strip <url> $E/leave --play --frames 1 --act .claude/skills/verify-tidepool/acts/leave-wheel.mjs`. The act clicks `Leave`, waits for `mode === 'watch'`, and turns the wheel without moving the pointer. It throws when the scale does not change within 500 ms.
- **Phone.** Repeat with `--size 390x844`. Check that the counter, the cards, the hint, and the buttons do not cover each other.

## Gotchas

- Bodies overlap each other in the sim. An overlap of two bodies is not a draw-order defect by itself. A body that flickers over and under another between frames is.
- The mate card and the bud card stand where each other would. Only one shows.
- When `Leave` goes, the `Guide` button slides under the pointer. The wheel there zoomed nothing until the wheel listener moved from the canvas to the window. A wheel over the guide or over a panel that has more to scroll still does not zoom.
