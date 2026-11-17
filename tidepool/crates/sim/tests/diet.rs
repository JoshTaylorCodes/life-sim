//! Rule D1 of the diet plan: the `diet` gene, the Filter factor `(1 - diet)^diet_k`, and the bite
//! factor `diet^diet_k`.

use tidepool_sim::genome::{BrainGenome, Genome, KIND_FILTER, KIND_JAW};
use tidepool_sim::math::pow_quarters;
use tidepool_sim::params::{id, Params};
use tidepool_sim::rng::Rng;
use tidepool_sim::world::{SpawnAt, World};

/// One body or two, held still, with no upkeep and plants that do not change.
fn still_water() -> World {
    let mut w = World::new(1, 512.0, 512.0);
    for k in [
        id::BASAL,
        id::FIXED_UPKEEP,
        id::NEURON_UPKEEP,
        id::CONN_UPKEEP,
        id::MOVE_COST,
        id::TURN_COST,
        id::REPEL,
        id::FLAG_FORCE,
        id::CILIA_FORCE,
        id::PLANT_GROWTH,
        id::PLANT_SEED_CHANCE,
        id::PLANT_DEATH_RESERVE,
        id::BITE_MASS_RATIO,
        id::SHELL_BLOCK,
        id::SPIKE_HURT,
    ] {
        w.params.set(k, 0.0);
    }
    w.params.set(id::STRIKE_BITE, 1.0);
    // Rule D2 off: these tests read rule D1 alone. `bite_across` sets it.
    w.params.set(id::BITE_DIET_GAP, -1.0);
    w
}

/// The reference founder at rest at the place, half full, with an empty brain. It never buds.
fn adult(w: &mut World, x: f32, diet: f32, jaw: bool) -> usize {
    adult_shaped(w, x, diet, |g| {
        if jaw {
            g.set_size(KIND_JAW, 0.5);
        }
    })
}

/// `adult`, with `shape` applied to the genome before the spawn.
fn adult_shaped(w: &mut World, x: f32, diet: f32, shape: impl FnOnce(&mut Genome)) -> usize {
    let mut g = Genome::founder(&mut Rng::new(1), &w.params);
    (g.len, g.wid, g.diet) = (12.0, 0.5, diet);
    g.brain = BrainGenome::default();
    shape(&mut g);
    let at = SpawnAt {
        x,
        y: 256.0,
        heading: 0.0,
        growth: 1.0,
        energy01: 0.5,
        parent: 0,
        generation: 0,
        mark: 0,
    };
    let slot = w.spawn(g, at).expect("slot") as usize;
    w.creatures.bud_cooldown[slot] = 1_000_000;
    slot
}

/// What a body of this diet takes from a plant it sits on in one tick. The plant holds more than
/// the body's Filter can take, so the Filter's rate decides the take.
fn filter_take(diet_k: f32, diet: f32) -> f32 {
    filter_size_take(diet_k, diet, 0.6)
}

/// `filter_take` for a body whose Filter has this size.
fn filter_size_take(diet_k: f32, diet: f32, size: f32) -> f32 {
    let mut w = still_water();
    w.params.set(id::DIET_K, diet_k);
    adult_shaped(&mut w, 200.0, diet, |g| g.set_size(KIND_FILTER, size));
    let s = w.plants.slots.alloc().expect("plant slot") as usize;
    (w.plants.x[s], w.plants.y[s]) = (200.0, 256.0);
    (w.plants.reserve[s], w.plants.cap[s]) = (40.0, 40.0);
    w.plants.dirty = true;
    w.tick();
    w.stats.eaten as f32
}

/// What a biter of this diet takes in one tick from a body 8 ahead of it. The prey holds more than
/// the bite, so the bite decides the take.
fn bite_take(diet_k: f32, diet: f32) -> f32 {
    let mut w = still_water();
    w.params.set(id::DIET_K, diet_k);
    adult(&mut w, 100.0, diet, true);
    adult(&mut w, 108.0, 0.0, false);
    w.tick();
    w.stats.flesh as f32
}

#[test]
fn with_diet_k_0_both_factors_are_1_at_every_diet() {
    let (filter, bite) = (filter_take(0.0, 0.0), bite_take(0.0, 1.0));
    assert!(filter > 0.0 && bite > 0.0);
    for diet in [0.0, 0.25, 0.5, 0.75, 1.0] {
        assert_eq!(filter_take(0.0, diet), filter, "diet {diet}");
        assert_eq!(bite_take(0.0, diet), bite, "diet {diet}");
    }
}

#[test]
fn a_diet_1_body_takes_nothing_from_a_plant_and_a_diet_0_body_bites_nothing() {
    assert!(filter_take(1.5, 0.0) > 0.0 && bite_take(1.5, 1.0) > 0.0);
    assert_eq!(filter_take(1.5, 1.0), 0.0);
    assert_eq!(bite_take(1.5, 0.0), 0.0);
}

#[test]
fn a_filter_of_size_0_eats_nothing_and_adds_no_mass() {
    assert!(filter_size_take(0.0, 0.0, 0.1) > 0.0);
    assert_eq!(filter_size_take(0.0, 0.0, 0.0), 0.0);

    let mut w = still_water();
    let (part_mass, eye_cost) = (w.params.get(id::PART_MASS), w.params.get(id::EYE_COST));
    let mass = |w: &World, s: usize| w.creatures.mass(s, part_mass, eye_cost);
    let none = adult_shaped(&mut w, 100.0, 0.0, |g| {
        g.parts.retain(|x| x.kind != KIND_FILTER)
    });
    let zero = adult_shaped(&mut w, 200.0, 0.0, |g| g.set_size(KIND_FILTER, 0.0));
    let full = adult_shaped(&mut w, 300.0, 0.0, |g| g.set_size(KIND_FILTER, 0.6));
    assert_eq!(mass(&w, zero), mass(&w, none));
    assert!(mass(&w, full) > mass(&w, zero));
}

#[test]
fn a_body_at_half_takes_half_to_the_diet_k_of_each_food() {
    let share = pow_quarters(0.5, 1.5);
    assert!((share - 0.353_553_4).abs() < 1e-6);
    assert_eq!(filter_take(1.5, 0.5), filter_take(1.5, 0.0) * share);
    assert_eq!(bite_take(1.5, 0.5), bite_take(1.5, 1.0) * share);
}

#[test]
fn founders_start_at_the_founder_diets() {
    let mut w = World::new(3, 1536.0, 1024.0);
    w.params.set(id::FOUNDER_JAW, 0.5);
    w.params.set(id::FOUNDER_DIET, 0.1);
    w.params.set(id::FOUNDER_HUNTER_DIET, 0.7);
    w.seed_founders(200, 0);
    let mut seen = [0; 2];
    for s in 0..w.creatures.slots.high_water() as usize {
        let hunter = w.genomes[s].size_of(KIND_JAW) > 0.0;
        let want = if hunter { 0.7 } else { 0.1 };
        assert_eq!(
            (w.genomes[s].diet, w.creatures.diet[s]),
            (want, want),
            "slot {s}"
        );
        seen[hunter as usize] += 1;
    }
    assert!(seen[0] > 0 && seen[1] > 0, "{seen:?}");
}

#[test]
fn a_mutation_keeps_the_diet_in_0_to_1_and_a_sigma_of_0_keeps_it() {
    let mut p = Params::new();
    p.set(id::DIET_SIGMA, 0.5);
    let mut rng = Rng::new(4);
    let mut next_history = 1000;
    for start in [0.0, 1.0] {
        let mut g = Genome {
            diet: start,
            ..Genome::default()
        };
        let mut moved = false;
        for _ in 0..200 {
            g = g.mutated(&mut rng, &p, &mut next_history);
            assert!((0.0..=1.0).contains(&g.diet), "{}", g.diet);
            moved |= g.diet != start;
        }
        assert!(moved);
    }
    p.set(id::DIET_SIGMA, 0.0);
    let g = Genome {
        diet: 0.3,
        ..Genome::default()
    };
    assert_eq!(g.mutated(&mut rng, &p, &mut next_history).diet, 0.3);
}

/// What a biter of `biter` diet takes in one tick from a body of `prey` diet, at `bite_diet_gap`
/// `gap` and `diet_k` 0, so only rule D2 can stop the bite.
fn bite_across(gap: f32, biter: f32, prey: f32) -> f32 {
    let mut w = still_water();
    w.params.set(id::DIET_K, 0.0);
    w.params.set(id::BITE_DIET_GAP, gap);
    adult(&mut w, 100.0, biter, true);
    adult(&mut w, 108.0, prey, false);
    w.tick();
    w.stats.flesh as f32
}

#[test]
fn a_body_bites_only_down_the_food_chain_by_the_gap() {
    // Rule D2: at a gap of 0.25 a biter of 0.8 bites a body of 0.5 and not one of 0.6.
    assert!(bite_across(0.25, 0.8, 0.5) > 0.0);
    assert_eq!(bite_across(0.25, 0.8, 0.6), 0.0);
    // At a gap of 0 an equal diet is bitten, and a higher one is not.
    assert!(bite_across(0.0, 0.5, 0.5) > 0.0);
    assert_eq!(bite_across(0.0, 0.5, 0.6), 0.0);
    // Off (−1): any body, even one higher on the chain, is bitten.
    assert!(bite_across(-1.0, 0.2, 0.9) > 0.0);
}

#[test]
fn a_spike_hurts_a_biter_in_step_with_its_bite() {
    // The spike hurt is the bite times `spike_hurt`, so a biter of diet 0.5 at `diet_k` 1.5 is hurt
    // 0.5^1.5 times what a biter of diet 1 is hurt.
    let hurt = |diet: f32| {
        let mut w = still_water();
        w.params.set(id::DIET_K, 1.5);
        w.params.set(id::SPIKE_HURT, 1.0);
        let s = adult(&mut w, 100.0, diet, true);
        let mut g = Genome::founder(&mut Rng::new(1), &w.params);
        // At the tail, which faces the biter behind it.
        g.parts.push(tidepool_sim::genome::PartGene {
            kind: tidepool_sim::genome::KIND_SPIKE,
            anchor: 1.0,
            size: 0.5,
            extra: 0.0,
            history: 900,
        });
        (g.len, g.wid, g.diet, g.brain) = (12.0, 0.5, 0.0, BrainGenome::default());
        let prey = w.spawn(
            g,
            SpawnAt {
                x: 108.0,
                y: 256.0,
                heading: 0.0,
                growth: 1.0,
                energy01: 0.5,
                parent: 0,
                generation: 0,
                mark: 0,
            },
        );
        w.creatures.bud_cooldown[prey.expect("slot") as usize] = 1_000_000;
        let before = w.creatures.energy[s];
        w.tick();
        before + w.stats.flesh as f32 - w.creatures.energy[s]
    };
    let (full, half) = (hurt(1.0), hurt(0.5));
    assert!(full > 0.0);
    assert!(
        (half / full - pow_quarters(0.5, 1.5)).abs() < 1e-4,
        "{half} / {full}"
    );
}

/// What a jawed biter of diet `biter` and length `biter_len` takes in one tick from a jawless body of
/// diet `prey` and length `prey_len`, at a gap of 0.25 and a `bite_mass_ratio` of 1.15.
fn top_bite(biter: f32, biter_len: f32, prey: f32, prey_len: f32) -> f32 {
    let mut w = still_water();
    w.params.set(id::DIET_K, 0.0);
    w.params.set(id::BITE_DIET_GAP, 0.25);
    w.params.set(id::BITE_MASS_RATIO, 1.15);
    adult_shaped(&mut w, 100.0, biter, |g| {
        g.set_size(KIND_JAW, 0.5);
        g.len = biter_len;
    });
    adult_shaped(&mut w, 108.0, prey, |g| g.len = prey_len);
    w.tick();
    w.stats.flesh as f32
}

#[test]
fn a_large_biter_at_the_top_of_diet_bites_a_smaller_body_near_the_top() {
    // Under a gap of 0.25 nothing is over 1, so before the top rule a body over 0.75 was immune.
    assert!(top_bite(1.0, 24.0, 0.9, 12.0) > 0.0);
    assert!(top_bite(0.9, 24.0, 1.0, 12.0) > 0.0);
}

#[test]
fn a_biter_that_fails_the_mass_ratio_cannot_bite_near_the_top() {
    assert_eq!(top_bite(1.0, 12.0, 0.9, 24.0), 0.0);
}

#[test]
fn below_the_top_a_biter_needs_the_gap_as_before() {
    // 0.8 + 0.25 is over 1, but a body of 0.6 is not within the gap of the top.
    assert_eq!(top_bite(0.8, 24.0, 0.6, 12.0), 0.0);
    // A biter of 0.7 is too low for the top rule and bites 0.45 by the gap alone.
    assert!(top_bite(0.7, 24.0, 0.45, 12.0) > 0.0);
    assert_eq!(top_bite(0.7, 24.0, 0.5, 12.0), 0.0);
}
