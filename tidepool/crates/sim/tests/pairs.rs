//! Finding 1 of the parts audit: a part gene off the midline makes a mirrored pair, and each side of
//! the pair gets half of the gene's size. A pair gives what one midline part of that size gives,
//! spread over two arcs.

use tidepool_sim::brain;
use tidepool_sim::genome::{
    BrainGenome, Genome, PartGene, BODY, KIND_EYE, KIND_JAW, KIND_SHELL, KIND_SPIKE,
};
use tidepool_sim::params::id;
use tidepool_sim::rng::Rng;
use tidepool_sim::world::{SpawnAt, World};

/// Near the nose: a pair at plus and minus 9 degrees, inside the arc of a jaw or an eye on either
/// side of the line ahead.
const NOSE_PAIR: f32 = 0.05;
/// Near the tail: a pair at plus and minus 171 degrees, inside the arc of a guard on either side of
/// the line behind.
const TAIL_PAIR: f32 = 0.95;

fn gene(kind: u8, anchor: f32, size: f32) -> PartGene {
    PartGene {
        kind,
        anchor,
        size,
        extra: 0.5,
        history: 900,
    }
}

/// Open water where the bite is the only rule that moves energy, as in `bite.rs`. The bite of a
/// jaw of size 0.5 on the reference body is `0.5 * 0.5 * 72 / 6` = 3.
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
        id::BITE_MASS_RATIO,
        id::SHELL_BLOCK,
        id::SPIKE_HURT,
        id::SHELL_DRAG,
        id::DIET_K,
    ] {
        w.params.set(k, 0.0);
    }
    w.params.set(id::BITE_RATE, 0.5);
    w.params.set(id::STRIKE_BITE, 1.0);
    w.params.set(id::BITE_DIET_GAP, -1.0);
    w
}

/// The reference founder at rest at `x`, heading along x, with an empty brain and the part added.
fn adult(w: &mut World, x: f32, part: Option<PartGene>) -> usize {
    adult_with(w, x, part.into_iter().collect())
}

fn adult_with(w: &mut World, x: f32, parts: Vec<PartGene>) -> usize {
    let mut g = Genome::founder(&mut Rng::new(1), &w.params);
    (g.len, g.wid) = (12.0, 0.5);
    g.brain = BrainGenome::default();
    g.parts.extend(parts);
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

/// A biter with the jaw at x 100, and a prey with the guard 8 ahead of it, so the biter is behind
/// the prey. Returns what the prey lost and the hurt of the biter after one tick.
fn bite(jaw: PartGene, guard: Option<PartGene>, set: &[(usize, f32)]) -> (f32, f32) {
    let mut w = still_water();
    for &(k, v) in set {
        w.params.set(k, v);
    }
    let s = adult(&mut w, 100.0, Some(jaw));
    let j = adult(&mut w, 108.0, guard);
    let ej = w.creatures.energy[j];
    w.run(1);
    (ej - w.creatures.energy[j], w.creatures.hurt[s])
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 1e-5
}

#[test]
fn a_pair_of_jaws_bites_as_one_midline_jaw_of_the_same_size() {
    let (midline, _) = bite(gene(KIND_JAW, 0.0, 0.5), None, &[]);
    let (pair, _) = bite(gene(KIND_JAW, NOSE_PAIR, 0.5), None, &[]);
    assert!(close(midline, 3.0), "{midline}");
    assert!(close(pair, 3.0), "{pair}");
}

#[test]
fn a_pair_of_spikes_hurts_as_one_midline_spike_of_the_same_size() {
    let hurt = |anchor: f32| {
        bite(
            gene(KIND_JAW, 0.0, 0.5),
            Some(gene(KIND_SPIKE, anchor, 0.5)),
            &[(id::SPIKE_HURT, 1.0)],
        )
        .1
    };
    // The bite is 3, and a spike of 0.5 hurts `1 * 0.5 * 3`.
    assert!(close(hurt(1.0), 1.5), "{}", hurt(1.0));
    assert!(close(hurt(TAIL_PAIR), 1.5), "{}", hurt(TAIL_PAIR));
}

#[test]
fn a_pair_of_shells_blocks_as_one_midline_shell_of_the_same_size() {
    let take = |anchor: f32, size: f32| {
        bite(
            gene(KIND_JAW, 0.0, 0.5),
            Some(gene(KIND_SHELL, anchor, size)),
            &[(id::SHELL_BLOCK, 1.0)],
        )
        .0
    };
    // A shell of 0.5 blocks half of the bite of 3.
    assert!(close(take(1.0, 0.5), 1.5), "{}", take(1.0, 0.5));
    assert!(close(take(TAIL_PAIR, 0.5), 1.5), "{}", take(TAIL_PAIR, 0.5));
    // A pair blocks a whole bite at the gene size `1 / shell_block`, where its drag stops rising,
    // as a midline shell does.
    assert_eq!(take(1.0, 1.0), 0.0);
    assert_eq!(take(TAIL_PAIR, 1.0), 0.0);
}

/// The plant input of each eye cone of a viewer with the eye, for a plant 10 ahead of it.
fn plant_inputs(eye: PartGene) -> Vec<f32> {
    let mut w = World::new(1, 1536.0, 1024.0);
    for k in [
        id::PLANT_GROWTH,
        id::PLANT_SEED_CHANCE,
        id::PLANT_DEATH_RESERVE,
        id::INTAKE_RATE,
        id::SCENT_DEPOSIT,
    ] {
        w.params.set(k, 0.0);
    }
    w.params.set(id::THINK_EVERY, 1.0);
    w.params.set(id::SEE_EVERY, 1.0);
    let p = w.plants.slots.alloc().expect("plant slot") as usize;
    (w.plants.x[p], w.plants.y[p]) = (710.0, 500.0);
    (w.plants.reserve[p], w.plants.cap[p]) = (20.0, 20.0);
    w.plants.dirty = true;
    let mut g = Genome::default();
    g.parts.push(eye);
    let at = SpawnAt {
        x: 700.0,
        y: 500.0,
        heading: 0.0,
        growth: 1.0,
        energy01: 1.0,
        parent: 0,
        generation: 0,
        mark: 0,
    };
    let s = w.spawn(g, at).expect("slot") as usize;
    w.tick();
    let inputs = brain::ports(&w.genomes[s]).inputs;
    let first = inputs
        .iter()
        .position(|n| n.owner != BODY)
        .expect("eye inputs");
    // Three inputs per cone: its plant, its body, its jaw.
    w.brains.inputs_mut(s)[first..]
        .iter()
        .step_by(3)
        .copied()
        .collect()
}

#[test]
fn each_eye_of_a_pair_sees_half_as_far_as_one_midline_eye_of_the_same_size() {
    // On the reference body an eye of size 0.2 sees `16 * 0.2 * 12` = 38.4 units.
    let midline = plant_inputs(gene(KIND_EYE, 0.0, 0.2));
    let pair = plant_inputs(gene(KIND_EYE, NOSE_PAIR, 0.2));
    assert_eq!(midline.len(), 1);
    assert!(close(midline[0], 1.0 - 10.0 / 38.4), "{midline:?}");
    // Each cone of the pair sees 19.2 units: the two reach as far as the one midline cone.
    assert_eq!(pair.len(), 2);
    for near in pair {
        assert!(close(near, 1.0 - 10.0 / 19.2), "{near}");
    }
}

/// The drag that one shell gene puts on a body coasting along its heading for one tick, at
/// `shell_block` 2: a side blocks a whole bite at the side size 0.5.
fn shell_drag(anchor: f32, size: f32) -> f32 {
    shells_drag(&[(anchor, size)])
}

/// The same for a body with a shell gene at each of the `(anchor, size)`.
fn shells_drag(shells: &[(f32, f32)]) -> f32 {
    let mut w = still_water();
    w.params.set(id::SHELL_BLOCK, 2.0);
    w.params.set(id::SHELL_DRAG, 0.1);
    let parts = shells
        .iter()
        .map(|&(anchor, size)| gene(KIND_SHELL, anchor, size))
        .collect();
    let s = adult_with(&mut w, 100.0, parts);
    w.creatures.vx[s] = 1.0;
    w.run(1);
    1.0 - w.creatures.vx[s]
}

/// The sides of a pair at 0.5 face right and left, so no direction holds both.
const SPLIT_PAIR: f32 = 0.5;

#[test]
fn a_split_pairs_drag_stops_rising_where_its_best_side_blocks_a_whole_bite() {
    // Each side holds half of the gene's size, so its best side blocks a whole bite at the size 1.
    let (before, at, past) = (
        shell_drag(SPLIT_PAIR, 0.8),
        shell_drag(SPLIT_PAIR, 1.0),
        shell_drag(SPLIT_PAIR, 1.4),
    );
    assert!(before < at - 1e-3, "{before} {at}");
    assert!(close(at, past), "{at} {past}");
}

#[test]
fn a_midline_shell_and_a_pair_that_holds_one_direction_stop_at_the_size_that_blocks_a_whole_bite() {
    // The sides of a nose pair both face the line ahead, so they block as one shell.
    for anchor in [1.0, TAIL_PAIR, NOSE_PAIR] {
        let (before, at, past) = (
            shell_drag(anchor, 0.4),
            shell_drag(anchor, 0.5),
            shell_drag(anchor, 0.9),
        );
        assert!(before < at - 1e-3, "{anchor}: {before} {at}");
        assert!(close(at, past), "{anchor}: {at} {past}");
    }
}

#[test]
fn a_full_shell_at_the_nose_and_one_at_the_tail_drag_as_two_whole_blocks() {
    // A biter is at one end, so the two shells never share a bite: each blocks a whole bite alone.
    let bare = shells_drag(&[]);
    let one = shells_drag(&[(0.0, 0.5)]) - bare;
    let both = shells_drag(&[(0.0, 0.5), (1.0, 0.5)]) - bare;
    assert!(one > 1e-3, "{one}");
    assert!(close(both, 2.0 * one), "{one} {both}");
}

#[test]
fn an_anchor_under_one_byte_step_is_a_midline_part_as_the_page_draws_it() {
    // The frame packs an anchor as a byte, and a page that reads the byte draws 0 as a midline
    // part. 0.001 is under half a step, 1/510.
    let cones = plant_inputs(gene(KIND_EYE, 0.001, 0.2));
    assert_eq!(cones.len(), 1);
    assert!(close(cones[0], 1.0 - 10.0 / 38.4), "{cones:?}");
    // 0.01 is over a step, so it stays a pair.
    assert_eq!(plant_inputs(gene(KIND_EYE, 0.01, 0.2)).len(), 2);
}
