use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{Destination, Home};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct GoHome;

pub fn go_home_action_system(
    mut cmd: Commands,
    homies: Query<(&Name, &Home)>,

    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &GoHome, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, _, span) in action_query.iter_mut() {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                // We don't really need any initialization code here, since the queries are cheap enough.

                // check if already at home
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_name, Home(home)) = homies.get(actor.0).expect("actor has no position");
                cmd.entity(actor.0).insert(Destination(*home));
                println!("{:?} is going home", actor_name);
                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
