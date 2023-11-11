use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{Hunger, Inventory};

// Scorers are the same as in the thirst example.
#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Hungry;

pub fn hungry_scorer_system(
    hungries: Query<(&Hunger, &Inventory)>,
    mut query: Query<(&Actor, &mut Score), With<Hungry>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok((hunger, inventory)) = hungries.get(*actor) {
            if inventory.food == 0 {
                score.set(0.);
                continue;
            }
            score.set(hunger.amount.clamp(0., 100.) / 100.);
        }
    }
}
