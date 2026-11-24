# Watch the world

The page opens on the baked world (`public/m2-world.tdpl`, tick 36,000) and the player watches the crowd through a fitted camera. A line says "Click any creature to become it."

## Sub-features

- `watch-open`: the page loads the baked world with no query.
- `watch-camera`: a drag pans, the wheel or a pinch zooms, a resize fits the world again while the camera is fitted.
- `watch-world-query`: `?world=<url>` loads another snapshot, `?founders` starts from founders at tick 0. A bad file shows the error panel with a fix.
- `watch-debug`: `?debug` shows the readout and the "Copy beat times" button. `?debug&p=<name>=<value>` sets a parameter.

## How to get to it (user POV)

- Open the dev URL with no query.
- Open it with `?world=m2-world-long.tdpl` (in `public/`, not in git) or `?founders`.

## Driving it with tidepool-verify

- **Open.** `handoff <url> $E/open`. Exit 0, `handoff.json` has `ready: true` and a tick near 36,000.
- **Motion.** `strip <url> $E/watch --every 20`. Bodies move between frames, plants stay rooted, no body jumps across the screen between two frames 20 ticks apart.
- **Bad world.** `handoff "<url>?world=nope.tdpl" $E/bad-world`. It must exit 1 with `alert` holding "did not load" and "Fix:". That is the error state working.
- **Camera.** In an act: `page.mouse.wheel(0, -400)` over the canvas, then read `window.__tidepool.camera.scale`. It rises.

## Gotchas

- The fitted camera at 1300x700 does not show the whole world: the scale stops at `SCALE_MIN` (0.4).
- `?founders` starts at tick 0. Pass `--from` or let the strip use 40.
