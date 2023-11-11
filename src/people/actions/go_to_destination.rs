use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::Destination;

pub const WALK_SPEED: f32 = 7.;
pub const ARRIVAL_DISTANCE: f32 = 3.0;

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct GoToDestination;

pub fn go_to_destination_action_system(
    //     mut cmd: Commands,
    time: Res<Time>,
    // We use Without to make disjoint queries.
    mut positions: Query<(&Name, &mut Transform, &Destination), With<Destination>>,

    destinations: Query<(&Transform, Option<&Name>), Without<Destination>>,
    // A query on all current MoveToWaterSource actions.
    mut action_query: Query<(&Actor, &mut ActionState, &GoToDestination, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (actor, mut action_state, _, span) in action_query.iter_mut() {
        let _guard = span.span().enter();
        let (actor_name, mut actor_position, destination) =
            positions.get_mut(actor.0).expect("actor has no position");

        let (destination_pos, destination_name) = destinations
            .get(destination.0)
            .expect("Destination does not exist");

        // Different behavior depending on action state.
        match *action_state {
            // Action was just requested; it hasn't been seen before.
            ActionState::Requested => {
                if destination.0 == actor.0 {
                    // walk to itself??
                    *action_state = ActionState::Success;
                    continue;
                }

                match destination_name {
                    Some(name) => println!("{:?} is going to {:?}", actor_name, name),
                    None => println!(
                        "{:?} is going somewhere we don't know because it has no name. But the Entity is {:?}",
                        actor_name,
                        destination.0
                    ),
                }
                // We don't really need any initialization code here, since the queries are cheap enough.
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                // Look up the actor's position.

                // Find how far we are from it.
                let delta = destination_pos.translation - actor_position.translation;

                let distance = delta.length();

                if distance > ARRIVAL_DISTANCE {
                    // We're still too far, take a step toward it!

                    // How far can we travel during this frame?
                    let step_size = time.delta_seconds() * WALK_SPEED;
                    // Travel towards the water-source position, but make sure to not overstep it.
                    let step = delta.normalize() * step_size.min(distance);

                    // Move the actor.
                    actor_position.translation += step;
                } else {
                    // We're within the required distance! We can declare success.
                    match destination_name {
                        Some(name) => println!("{:?} arrived at {:?}", actor_name, name),
                        None => println!("{:?} arrived at destination", actor_name),
                    }

                    // cmd.entity(actor.0).remove::<Destination>();

                    // The action will be cleaned up automatically.
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                // We're within the required distance! We can declare success.
                match destination_name {
                    Some(name) => println!("{:?} pauses going to {:?} because he has something more important to do..", actor_name, name),
                    None => println!("{:?} stopped in his tracks because he has something more important to do..", actor_name),
                }
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
