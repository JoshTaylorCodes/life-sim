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
