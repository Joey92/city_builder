use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{CurrentActivity, Destination, Workplace};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct GoWork;

pub fn go_work_action_system(
    mut cmd: Commands,
    homies: Query<(&Name, &CurrentActivity, &Workplace)>,

    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &GoWork, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, _, span) in action_query.iter_mut() {
        let _guard = span.span().enter();

        let (actor_name, activity, Workplace(work)) =
            homies.get(actor.0).expect("actor has no position");

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = match activity {
                    CurrentActivity::Sleeping => ActionState::Failure,
                    _ => ActionState::Executing,
                }
            }
            ActionState::Executing => {
                cmd.entity(actor.0).insert(Destination(*work));
                println!("{:?} is going to work", actor_name);
                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
