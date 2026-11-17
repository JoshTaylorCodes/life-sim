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
- Long gate 1/5 (seed 4 passes): eye gap 0.089, 0.068, 0.087, −0.062 on seeds 1, 5, 6, 8. Every
  other band passes. (Corrected 2026-11-17 by #1708: this line first named seed 5 as the pass.)

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

### The eye gap per seed, both heads (#1708)

`eye` and `eye_blind` are the share of bodies with an eye in the world and in its blind arm, as the
long gate prints them (two places). The gap is the gate's own figure where it failed; on a pass it
is 0.10 or more, shown as the difference of the printed shares.

| Seed | 144,000 at `76e3cc60` | Gap | 144,000 at `225720e6` | Gap |
|---|---|---|---|---|
| 1 | 0.12 / 0.04 | 0.089 FAIL | 0.19 / 0.01 | ≈0.18 (fails the chemo gap) |
| 4 | 0.24 / 0.06 | ≈0.18 PASS | 0.33 / 0.02 | ≈0.31 PASS |
| 5 | 0.11 / 0.04 | 0.068 FAIL | 0.06 / 0.04 | 0.027 FAIL |
| 6 | 0.13 / 0.04 | 0.087 FAIL | 0.29 / 0.03 | ≈0.26 PASS |
| 8 | 0.03 / 0.09 | −0.062 FAIL | 0.09 / 0.02 | 0.074 FAIL |

At 36,000 (seeds 1 to 3): `76e3cc60` 0.020, −0.117, 0.022, all FAIL; `225720e6` ≈0.17, ≈0.13 PASS
and 0.078 FAIL.

**A change inside the batch that is not a fix:** `founder_eye` rose from 0.2 to 0.4 with #1677, so
each cone of the founders' eye pair keeps its old size. The founder pays mass for both cones, so
founder mass rises (the debris test's built mass went from 76.7 to 81.3). The batch reading
includes it.

## Batch: `225720e6`

2026-11-17, 00:28 to 00:58, load 8 to 11 (other projects' tests; deterministic runs). `make check`
exit 0. Full log: `rows/parts-batch-225720e6.log`. The batch: #1677 to #1686 and #1692, #1695.
New params at their unmeasured defaults: `turn_cost` 0.003, `chemo_floor` 0.1,
`chemo_level_floor` 0.015, `eye_trade` 1.0; `dash_cost` 10 and `toxin_cost` 48 per part size;
`founder_eye` 0.4. Format 17.

- `gate-health` 3/3 PASS (baseline 3/3).
- `gate-evolve` **2/3** (baseline 0/3): seed 3 eye gap 0.078. Eyes at 36,000 rose to 0.25 to 0.30 of
  bodies (baseline 0.07 to 0.13).
- Long gate **2/5** (baseline 1/5), seeds 4 and 6 pass. Seed 1 fails a band the baseline passed:
  **chemo gap −0.006** (chemoreceptor in 0.09 of bodies, 0.10 of the blind arm; the baseline's
  seed 1 had 0.52). Seeds 5 and 8 fail the eye gap (0.027, 0.074).
- Chemoreceptors at 144,000 fell on every seed: 0.09, 0.44, 0.18, 0.28, 0.27 (baseline 0.41 to
  0.52). The likeliest cause is the new chemo floor (#1685): a receptor under a size gives nothing.
  Not tested: no run isolates it.

`probe-diet` at 144,000:

| Seed | Creatures | Herb | Omni | Carn | Hunter share | Herb with a jaw | `prey_lo` |
|---|---|---|---|---|---|---|---|
| 1 | 777 | 0.336 | 0.517 | 0.147 | 0.764 | 0.356 | 183 |
| 4 | 1,109 | 0.775 | 0.191 | 0.033 | 0.786 | 0.786 | 175 |
| 5 | 669 | 0.291 | 0.538 | 0.170 | 0.774 | 0.405 | 25 |
| 6 | 926 | 0.739 | 0.211 | 0.051 | 0.641 | 0.547 | 332 |
| 8 | 497 | 0.235 | 0.531 | 0.233 | 0.950 | 0.812 | 6 |

At 36,000 on seed 1, the age of the baked world: hunter share **0.328** (baseline 0.899),
herbivores with a jaw 0.325 (baseline 0.888). A jaw marks a hunter again in the world the page opens on.

Against #1690's target: omnivores 0.50 or under on seeds 4 and 6 (baseline none), but their
carnivores are 0.033 and 0.051. No seed meets every band (baseline none). Hunters stay under 0.8 at
the end on seeds 1, 4, 5, 6 (baseline none).

**Verdict:** the batch moves every diet and hunter measure toward the target and gates 36,000
better, and the gates stay red: the long gate 2/5 with a new chemo gap failure on seed 1. By the
constitution of 2026-11-16 the batch ships on, and this goes to the owner. Throughput was not read:
the machine held other projects' tests (load 8 to 11); it is read on a quiet machine before the
rebake (#1691).
