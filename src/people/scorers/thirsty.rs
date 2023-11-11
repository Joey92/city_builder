use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{Inventory, Thirst};

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Thirsty;

pub fn thirsty_scorer_system(
    thirsts: Query<(&Thirst, &Inventory)>,
    mut query: Query<(&Actor, &mut Score), With<Thirsty>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok((thirst, inventory)) = thirsts.get(*actor) {
            if inventory.drink == 0 {
                score.set(0.);
                continue;
            }
            score.set(thirst.amount.clamp(0., 100.) / 100.);
        }
    }
}
