use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{Hunger, Inventory, Thirst};

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Idling {
    pub cooldown: Timer,
}

pub fn idle_scorer_system(
    time: Res<Time>,
    procrastinators: Query<(&Thirst, &Hunger, &Inventory)>,
    mut query: Query<(&Actor, &mut Score, &mut Idling)>,
) {
    for (Actor(actor), mut score, mut action) in query.iter_mut() {
        if let Ok((thirst, hunger, inventory)) = procrastinators.get(*actor) {
            if !action.cooldown.tick(time.delta()).finished() {
                score.set(0.);
                continue;
            }

            if thirst.amount < 50.
                && hunger.amount < 50.
                && inventory.food > 2
                && inventory.drink > 2
            {
                score.set(0.7);
                continue;
            }
            score.set(0.);
        }
    }
}
