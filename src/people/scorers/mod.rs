use bevy::prelude::*;

use self::{
    hungry::hungry_scorer_system, idling::idle_scorer_system,
    inventory_low::inventory_scorer_system, sleepy::sleepiness_scorer_system,
    thirsty::thirsty_scorer_system, work::work_scorer_system,
};

pub mod hungry;
pub mod idling;
pub mod inventory_low;

pub mod sleepy;
pub mod thirsty;
pub mod work;

pub struct ScorerPlugin;

impl Plugin for ScorerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (
                hungry_scorer_system,
                thirsty_scorer_system,
                inventory_scorer_system,
                work_scorer_system,
                idle_scorer_system,
                sleepiness_scorer_system,
            ),
        );
    }
}
