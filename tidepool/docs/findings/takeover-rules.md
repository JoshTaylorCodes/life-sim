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
