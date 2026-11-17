# The parts batch (#1677 to #1686, #1692): rows

Spec #1670, ticket #1687. The nine fixes of `parts-audit.md` (all but finding 4) as one batch,
read against the head before the batch by the same protocol. The bar is green gates, not "helps".
One long run at a time, every `--param` at its default.

Protocol: `gate-health` and `gate-evolve` (default ticks, seeds 1 to 3), `probe-diet` at 12,000
(seeds 1 to 3) and at 144,000 (seeds 1, 4, 5, 6, 8, every 12,000), then the long gate
(`gate-evolve --seeds 1,4,5,6,8 --ticks 144000 --brain --predation`).

## Baseline: `76e3cc60`

2026-11-16, 23:06 to 23:33, load 2.7 at the start and up to 29 by the end (other agents' builds;
these runs are deterministic, so the load changes only the time). Full log:
`rows/parts-batch-baseline-76e3cc60.log`.

- `gate-health` 3/3 PASS.
- `gate-evolve` 0/3: eye gap 0.020, −0.117, 0.022 against 0.10. Every other band passes.
- Long gate 1/5 (seed 5 passes): eye gap 0.089, 0.068, 0.087, −0.062 on seeds 1, 4, 6, 8. Every
  other band passes.

`probe-diet` at 144,000 (herbivore under 0.3, carnivore over 0.7; hunter = a body with a jaw):

| Seed | Creatures | Herb | Omni | Carn | Hunter share | Herb with a jaw |
|---|---|---|---|---|---|---|
| 1 | 606 | 0.046 | 0.741 | 0.213 | 0.990 | 1.000 |
| 4 | 904 | 0.501 | 0.413 | 0.086 | 0.985 | 0.978 |
| 5 | 643 | 0.207 | 0.516 | 0.277 | 1.000 | 1.000 |
| 6 | 739 | 0.309 | 0.571 | 0.120 | 0.940 | 0.864 |
| 8 | 500 | 0.182 | 0.532 | 0.286 | 0.828 | 0.978 |

At 36,000 on seed 1, the age of the baked world: hunter share 0.899, herbivores with a jaw 0.888.
A jaw is near universal. "Hunter" (a jaw) no longer tells a hunter from prey in the shipped world.
Against #1690's target (omnivores 0.50 or under, herbivores and carnivores 0.10 or more each,
hunters never over 0.8 of the living): no seed meets the diet bands (seed 4 is nearest, carnivores 0.086) and no seed meets the hunter band.

## Batch: (pending)
