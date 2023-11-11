use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{building::store::Store, people::Destination};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct FindClosestStore;

pub fn find_closest_store_action_system(
    mut cmd: Commands,

    // We use Without to make disjoint queries.
    things: Query<(Entity, &Transform), With<Store>>,

    actors: Query<(&Name, &Transform)>,
    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &FindClosestStore, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, _, span) in action_query.iter_mut() {
        let _guard = span.span().enter();

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.
                let (actor_name, actor_position) =
                    actors.get(actor.0).expect("actor has no position");

                let closest_thing = things
                    .iter()
                    .min_by(|(_, a), (_, b)| {
                        let da = (a.translation - actor_position.translation).length_squared();
                        let db = (b.translation - actor_position.translation).length_squared();
                        da.partial_cmp(&db).unwrap()
                    })
                    .expect("no store found");

                cmd.entity(actor.0).insert(Destination(closest_thing.0));
                println!("{:?} going to the store", actor_name);
                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                // Always treat cancellations, or we might keep doing this forever!
                // You don't need to terminate immediately, by the way, this is only a flag that
                // the cancellation has been requested. If the actor is balancing on a tightrope,
                // for instance, you may let them walk off before ending the action.
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
