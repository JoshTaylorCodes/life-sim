# In-page bench

`?bench` measures this device: after the world loads, the worker settles a bench world for 12,000 ticks (`BENCH_SETTLE_TICKS`), times 2,000 more, and the page shows one line in `p.bench`: "n ticks per second at m creatures. A 24,000-tick skip would take s seconds on this device." Below 1,200 creatures it adds that the reading is too fast. The main world does not owe the bench's time.

## Sub-features

- `bench-measure`: the line "Measuring. This can take a minute on a slow device." shows until the answer.
- `bench-result`: the answer line, and the too-fast note below `BENCH_MIN_CREATURES`.

## How to get to it (user POV)

- Open the page with `?bench`. It has no button.

## Driving it with tidepool-verify

- **Measure.** `handoff "<url>?bench" $E/bench --act .claude/skills/verify-tidepool/acts/bench.mjs`. The act checks that `p.bench` shows, then waits up to 240 s for the answer, and throws if the line does not read as a result. Exit 0 and the line is printed.

## Gotchas

- `handoff` waits 3 s before the act. On 2026-11-24 an act that waited 5 s for "Measuring." after that wait did not find it, and the next run read the answer at once. The cause was not traced. To check the "Measuring." line, take a screenshot early in your own act.

- A headless Chromium on a loaded machine reads slowly. The number is not a throughput gate: `make gates` runs `scripts/bench-wasm.mjs` for that. Do not quote this reading as a measurement of the device.
