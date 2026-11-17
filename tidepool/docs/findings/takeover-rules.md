# The takeover rules (#1688 I1, #1689 O1), tuned by #1690: rows

Spec #1670. Target at tick 144,000 on 4 of 5 seeds (1, 4, 5, 6, 8): omnivores 0.50 or under,
herbivores and carnivores 0.10 or more each, and hunters (a jaw) never over 0.8 of the living,
read as the most of the 12,000-tick samples from tick 72,000 on. A cap of 8 trials. Each trial's
hypothesis is committed before its run. One run at a time. Protocol per trial: `gate-health`
(12,000, seeds 1 to 3) and `probe-diet` at 144,000. Scripts: `rows/takeover-trial.sh`,
`rows/takeover-score.py`. A miss ships on and goes to the owner (constitution, 2026-11-16).

## Baseline: the parts batch, `225720e6` (rows in `parts-batch.md`)

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.336 | 0.517 | 0.147 | 0.764 | misses (omni) |
| 4 | 0.775 | 0.191 | 0.033 | 0.823 | misses (carn, hunters) |
| 5 | 0.291 | 0.538 | 0.170 | 0.969 | misses |
| 6 | 0.739 | 0.211 | 0.051 | 0.641 | misses (carn) |
| 8 | 0.235 | 0.531 | 0.233 | 0.982 | misses |

0 of 5. `gate-health` 3/3.

## T1: both rules at their defaults (`23c44754`): hypothesis before the run

`interference` 0.3, `interference_radius` 48, `jaw_filter_cost` 1.0. Expected: O1 pushes jawed
bodies off plants, so omnivores fall and carnivores rise on seeds 4 and 6; I1 caps the hunter share
under 0.8 on seeds 5 and 8. Risk: O1 starves the founder hunters (jaw 0.5 keeps 2/3 of its
Filter) and carnivores vanish early.

### T1 rows

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.939 | 0.061 | 0.000 | 0.495 | misses (carn) |
| 4 | 0.949 | 0.051 | 0.000 | 0.360 | misses (carn) |
| 5 | 0.949 | 0.051 | 0.000 | 0.172 | misses (carn) |
| 6 | 0.946 | 0.054 | 0.000 | 0.124 | misses (carn) |
| 8 | 0.933 | 0.066 | 0.001 | 0.648 | misses (carn) |

0 of 5, `gate-health` 3/3. **The risk came true:** hunting collapses. Kills fall from about 500 at
12,000 to under 10 by 36,000 on seeds 1 and 5, carnivores are gone everywhere, and the world is
plant eaters. The hunter band holds only because there is little to hunt with. Log: `rows/takeover-T1.log`.

## T2: I1 alone (`jaw_filter_cost` 0, `interference` 0.3): hypothesis before the run

Isolates which rule ended the hunt. Expected: I1 alone acts only where hunters crowd, so carnivores
survive (0.1 to 0.25 as in the baseline) and the hunter share on seeds 5 and 8 falls under 0.8. If
carnivores still vanish, I1 at 0.3 is the cause.

### T2 rows

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.946 | 0.054 | 0.000 | 0.188 | misses (carn) |
| 4 | 0.952 | 0.048 | 0.000 | 0.828 | misses (carn, hunters) |
| 5 | 0.941 | 0.059 | 0.000 | 0.213 | misses (carn) |
| 6 | 0.950 | 0.050 | 0.000 | 0.264 | misses (carn) |
| 8 | 0.955 | 0.045 | 0.000 | 0.479 | misses (carn) |

0 of 5. **I1 alone ends the hunt too.** On seed 1 the hunter share is 0.010 at 12,000: the founder
hunters fail before they establish. They start near each other, and at a radius of 48 every
founder hunter counts its neighbours. Log: `rows/takeover-T2.log`.

## T3: I1 alone at a radius of 16 (`interference` 0.3, `interference_radius` 16, `jaw_filter_cost` 0): hypothesis before the run

16 is about bite reach, so only hunters at the same prey count, which is what interference is in
the literature (hunters competing over one kill). Expected: founder hunters establish (hunter share
at 12,000 near the baseline's 0.3), carnivores survive, and the share stays under 0.8 on seeds 5 and 8.

### T3 rows

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.937 | 0.063 | 0.000 | 0.568 | misses (carn) |
| 4 | 0.922 | 0.078 | 0.000 | 0.511 | misses (carn) |
| 5 | 0.400 | 0.480 | 0.121 | 0.914 | misses (hunters) |
| 6 | 0.791 | 0.178 | 0.031 | 0.857 | misses (carn, hunters) |
| 8 | 0.419 | 0.511 | 0.070 | 0.785 | misses (omni, carn) |

0 of 5. The hunt returns (seed 5: 733 kills at 36,000, 5,196 at 72,000), and seed 5 meets every
diet band, but hunters peak over 0.8 there. Seeds 1 and 4 lose the carnivores the batch head had
(seed 1: 0.147 at the head). I1 at bite range still thins carnivores and does not hold the hunter
share. Log: `rows/takeover-T3.log`.

## T4: O1 alone at 0.3 (`jaw_filter_cost` 0.3, `interference` 0): hypothesis before the run

O1 is aimed at the middle: a jawed grazer loses grazing, so it should go one way or the other.
At 0.3 a founder jaw of 0.5 keeps 87% of its Filter. Expected: omnivores fall under 0.50 on seeds
1, 5, 8 (0.52 to 0.54 at the head), and carnivores stay near the head's 0.15 to 0.23. The hunter
share is not O1's job: it may stay over 0.8 on seeds 5 and 8.

### T4 rows

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.937 | 0.063 | 0.000 | 0.653 | misses (carn) |
| 4 | 0.925 | 0.075 | 0.000 | 0.267 | misses (carn) |
| 5 | 0.493 | 0.424 | 0.083 | 0.859 | misses (carn, hunters) |
| 6 | 0.941 | 0.059 | 0.000 | 0.290 | misses (carn) |
| 8 | 0.940 | 0.060 | 0.000 | 0.166 | misses (carn) |

0 of 5. **O1 alone, even at 0.3, keeps carnivores from evolving on 4 of 5 seeds.** Kills fall from
about 700 at 12,000 to under 60 by 36,000 (seed 1), as in T1 and T2. The expectation is falsified.

**What T1 to T4 show together.** In this world carnivores do not arrive with the founders: the
founder hunters fade by 36,000, and carnivores evolve late (the head's seed 1: 0.016 at 36,000,
0.147 at 144,000) by a climb from jawed grazers through the middle. Each rule cuts the climb:
O1 taxes the middle step, I1 taxes the payoff of the hunt. The head is already close on seeds 1,
5 and 8 (omnivores 0.52 to 0.54, carnivores in band). Log: `rows/takeover-T4.log`.

## T5: both rules light (`interference` 0.1 at radius 16, `jaw_filter_cost` 0.1): hypothesis before the run

A small dose that trims the middle and caps the hunter share without cutting the climb. Expected:
carnivores at or near the head's (0.15 to 0.23 on seeds 1, 5, 8), omnivores 0.02 to 0.05 lower,
under 0.50 on two of those seeds; hunter peak under 0.8 on seed 5 or 8. If carnivores still
vanish, no dose of these two rules keeps the climb, and the target needs a different rule.

### T5 rows

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.945 | 0.054 | 0.001 | 0.541 | misses (carn) |
| 4 | 0.962 | 0.038 | 0.000 | 0.239 | misses (carn) |
| 5 | 0.947 | 0.053 | 0.000 | 0.309 | misses (carn) |
| 6 | 0.920 | 0.079 | 0.001 | 0.445 | misses (carn) |
| 8 | 0.919 | 0.081 | 0.000 | 0.123 | misses (carn) |

0 of 5. No carnivores on any seed, even at a tenth of each rule. Log: `rows/takeover-T5.log`.

## T6: control, both rules at 0 on `23c44754`: hypothesis before the run

That even a light dose removes every carnivore is suspicious. With `interference` 0 and
`jaw_filter_cost` 0 both rules are off to the bit, so this head should give the batch head's rows
(`225720e6`) exactly. If it does, the rules cut the climb at any dose. If it does not, something
else moved the world between the two heads, and T1 to T5 compare against the wrong baseline.

### T6 rows

Row for row the batch head's (`225720e6`): seed 1 777 bodies, 0.336, 0.517, 0.147, and so on for
every seed and sample. Both rules at 0 are off to the bit. **The rules cut the carnivores, at every
dose tried.** O1 cuts them at 0.1 (T5), 0.3 (T4) and 1.0 (T1). I1 at a radius of 48 cuts them
(T2). Only I1 at bite range, 16, kept carnivores, on 2 seeds (T3). Log: `rows/takeover-T6.log`.

## T7: I1 at bite range, stronger (`interference` 0.6, radius 16, `jaw_filter_cost` 0): hypothesis before the run

T3 kept carnivores on seeds 5 and 8 and met the diet bands on seed 5, but hunters peaked at 0.91.
At 0.6, two hunters at one prey take 0.45 each of a lone hunter's bite. Expected: the hunter peak
on seeds 5 and 8 falls under 0.8, carnivores hold over 0.10 on one of them at least. Risk: as at
radius 48, the hunt fails.

### T7 rows

| Seed | Herb | Omni | Carn | Hunters, most from 72,000 | Target |
|---|---|---|---|---|---|
| 1 | 0.942 | 0.058 | 0.000 | 0.573 | misses (carn) |
| 4 | 0.947 | 0.052 | 0.001 | 0.505 | misses (carn) |
| 5 | 0.935 | 0.065 | 0.000 | 0.847 | misses (carn, hunters) |
| 6 | 0.942 | 0.058 | 0.000 | 0.426 | misses (carn) |
| 8 | 0.306 | 0.468 | 0.225 | 0.945 | misses (hunters) |

0 of 5. Seed 8 meets every diet band, and its hunters peak at 0.945. Carnivores on 1 seed (T3: 2).
Log: `rows/takeover-T7.log`.

## T8: the set to ship: T3 with O1 on at a token dose (`interference` 0.3, radius 16, `jaw_filter_cost` 0.03): hypothesis before the run

The last trial. Measures the shipped set itself. A rule cannot ship off (constitution, 2026-11-16), and
O1 cut the carnivores at every dose read (0.1 to 1.0), so it ships at 0.03 and its deletion goes to
the owner. Expected: near T3 (carnivores on seeds 5 and 8, seed 5 in the diet bands), 0 or 1 of 5.
