use bevy::prelude::*;
use big_brain::BigBrainSet;

use self::{
    buy_stuff::buy_stuff_action_system, drink::drink_action_system, eat::eat_action_system,
    find_nearest_store::find_closest_store_action_system, go_home::go_home_action_system,
    go_to_destination::go_to_destination_action_system, go_work::go_work_action_system,
    work::work_action_system,
};

pub mod buy_stuff;
pub mod drink;
pub mod eat;
pub mod find_nearest_store;

pub mod go_home;
pub mod go_to_destination;
pub mod go_work;
pub mod work;

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            PreUpdate,
            (
                drink_action_system,
                eat_action_system,
                go_to_destination_action_system,
                buy_stuff_action_system,
                go_home_action_system,
                find_closest_store_action_system,
                go_work_action_system,
                work_action_system,
            )
                .in_set(BigBrainSet::Actions),
        );
    }
}
