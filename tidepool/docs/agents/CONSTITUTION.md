# Tidepool constitution

The non-negotiable rules for `tidepool/`. Every agent that plans, implements, or reviews Tidepool work reads this file in full. A breach is a hard violation, never a judgement call. The reasoning behind each rule is in `docs/architecture.md` and `docs/decisions/INDEX.md`. This file states the rules only.

## Verification

Run from `tidepool/`. If `cargo` is not found, run `. "$HOME/.cargo/env"` first.

```sh
make check
```

- `make check` must exit 0 before any task is done. It runs the crate count, format, Clippy, Rust tests, the WASM build, the type check, ESLint, and Vitest.
- At a milestone exit, `make gates` and `make gates-long` also run. Their result goes in the exit record under `docs/findings/`, red or green. They are not per-task checks.
- A gate counts only after it has rejected a known-bad input. Record the proof in `docs/gates.md`.
- Claim only what was measured. "The mechanism fires", "evolution uses it", and "it helps" are three claims, and each needs its own measurement.
- Tune at 12,000 ticks on 3 seeds against a baseline. Run one long simulation at a time.

## Architectural boundaries

- `crates/sim` has zero dependencies. `crates/headless` imports only `sim`. The workspace holds only these two crates.
- `src/worker` imports `protocol` and the `.wasm`. `src/render` and `src/ui` import `protocol` only and never each other or `worker`. `src/app` imports all of them, and nothing imports `app`.
- `tidepool/` shares no code, config, or dependencies with the draft in the repo root. Never import from `/src` and never edit it from Tidepool work.
- `spikes/` is throwaway. Never import from it. Copy code out of it.
- Same rules for everyone: the player's creature runs on the same data and code as every other creature. No player-only systems.

## Determinism

- The same build, seed, parameters, and inputs give the same state hash in native and in WASM.
- No standard-library transcendental math in `crates/sim`: trigonometric and hyperbolic functions and their inverses, `exp`, logarithms, `powf`, `powi`, `hypot`, `cbrt`, and `mul_add`. Use `sim::math`. `sqrt`, `abs`, and rounding are allowed.
- One seeded generator, forked into one stream per subsystem. Iterate in ascending slot order. No hash-map iteration.
- A field that affects the future is in `World::save` and `World::load`. A field missing from them is a bug. The format version rises on any layout change.
- A load of bad data returns an error, never a half-built world.

## Forbidden

- No magic numbers in rule code. A new tunable constant goes in `params.rs` with a default, a minimum, and a maximum. (The 14 older groups listed in `docs/findings/m3-audit.md` step 6 are known debt, not precedent.)
- No dead flags. A rule that fails its target ships on and its finding goes to the owner, or the owner deletes it. Never keep a rule behind an off flag. The note stays in `docs/findings/` (owner, 2026-11-16).
- No reopening a decision in `docs/decisions/INDEX.md` without new evidence. A decision marked "provisional, owner to confirm" is not final.
- TypeScript stays on 6. `typescript-eslint` 8 requires `<6.1.0`.
- No push in the middle of a plan. Commit at each task.

## Diff-triggered checks

`/converge` runs only the checks whose trigger matches the diff. Add one when a defect reached the user that the diff could have flagged (AGENTS.md, The loop, step 6). Delete one that has not fired in three months.

The verify skill is `.claude/skills/verify-tidepool/SKILL.md` at the repo root. `strip` and `handoff` below are its commands.

| Trigger (readable from a diff) | Check | Origin |
|---|---|---|
| A change to what `World::save` writes, to the gene or part layout in `crates/sim/src/genome.rs`, or to a rule that a saved world depends on. Also a merge where both sides raised `FORMAT_VERSION`. | `FORMAT_VERSION` in `crates/sim/src/snapshot.rs` is one above the value on the target branch's head now (`git show <base>:tidepool/crates/sim/src/snapshot.rs`), not above the value when the branch started. Rebake `public/m2-world.tdpl`, then run the verify skill's `doctor`. | `62cc4cc2`, `a484bc7d`, `c96b6f39`, `dfa0da20` |
| A change to `tidepool/index.html` styles (`position`, `top`, `bottom`, `width`, `max-height`, media queries) or to where `src/ui/App.tsx` places an overlay, card, panel, or the guide. | `strip --play` at `--size 390x844` and at `1300x700`, with `acts/brain.mjs` and with `acts/guide.mjs`. The hint line, the counter, the bud and mate cards, the brain sheet, the guide, and the death overlay do not cover each other or the possessed body. | `e5137ce1`, `2c022ac0`, `d7f0a1ac` |
| A change to `src/render/parts.ts`, `src/render/shaders.ts`, or the instance data in `src/render/renderer.ts`. | `strip` in Watch (small bodies at the fitted scale) and `strip --play` (scale 4). Each body keeps its parts at both scales, and a dim body dims its parts. | `1997e89d`, `587115b8` |
| A change to a comparison in `crates/sim/src/bite.rs`, or to the kill list in `world.rs`, `frame.rs`, or `abi.rs`. | A test in `crates/sim/tests/teeth.rs` or `tests/bite.rs` at the exact boundary value and for the same-tick case: a killer that dies in the same kill list, a body bitten by its own jaw. | `089ad3ac`, `1afdfddf`, `fbc3f662`, `23c44754` |
| A change to the pass rule or a band in `crates/headless/src/evolve.rs` or `health.rs`. | Run the gate on a known-bad input (a dead world, a band edge) and see it fail. Record the proof in `docs/gates.md`. | `8ad8efb7`, `bb383e50` |
| A change to a default, minimum, or maximum in the `table!` of `crates/sim/src/params.rs`. | At the new minimum and the new maximum, each rule that reads the parameter stays inside its own range: no negative cost, no gene outside its range. Add the case to `crates/sim/tests/params.rs` or the rule's test. | `903d99d2`, `a5c332c0` |
| A number or a "this works" sentence added to `docs/findings/`, `docs/plans/`, an exit record, or `src/ui/words/guide.ts`. | Each number names the row or command that measured it. Each guide sentence is true of the shipped rules, and `tests/guide.test.ts` passes. | `1a1cb452`, `b831cdb4`, `283d0c23`, `a5206200` |
