use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::Sleep;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Sleepy;

pub fn sleepiness_scorer_system(
    sleeps: Query<&Sleep>,
    mut query: Query<(&Actor, &mut Score), With<Sleepy>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(tired) = sleeps.get(*actor) {
            if tired.amount > 100. {
                score.set(1.);
                continue;
            }

            score.set(tired.amount / 100.);
        }
    }
}
