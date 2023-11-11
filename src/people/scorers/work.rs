use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    people::{CurrentActivity, Workplace},
    work::AvailableWorkers,
};

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct TimeToWork;

pub fn work_scorer_system(
    workers: Query<(&Workplace, &CurrentActivity)>,
    workplaces: Query<&AvailableWorkers>,
    mut query: Query<(&Actor, &mut Score), With<TimeToWork>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok((workplace, state)) = workers.get(*actor) {
            if CurrentActivity::Sleeping == *state {
                score.set(0.);
                continue;
            }

            // check workplace working times
            if let Ok(work) = workplaces.get(workplace.0) {
                score.set(1. - (work.current_workers / work.max_workers) as f32);
                continue;
            }

            score.set(0.);
        }
    }
}
