use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{Hunger, Inventory};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Eat;

pub fn eat_action_system(
    mut hungry: Query<(&Name, &mut Hunger, &mut Inventory)>,
    mut query: Query<(&Actor, &mut ActionState, &Eat, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, _, span) in query.iter_mut() {
        let _guard = span.span().enter();

        // Look up the actor's position and thirst from the Actor component in the action entity.
        let (actor_name, mut hunger, mut inventory) =
            hungry.get_mut(*actor).expect("actor has no hunger");

        match *state {
            ActionState::Requested => {
                // We'll start drinking as soon as we're requested to do so.
                println!("{:?} wants to eat food", actor_name);
                if inventory.food == 0 {
                    println!("{:?} has no more food in inventory", actor_name);
                    *state = ActionState::Failure;
                    continue;
                }
                inventory.food -= 1;
                *state = ActionState::Executing;
            }
            ActionState::Executing | ActionState::Cancelled => {
                hunger.amount -= 0.5;

                if hunger.amount <= 0. {
                    println!("{:?} ate enough food", actor_name);
                    *state = ActionState::Success;
                }
            }
            _ => {}
        }
    }
}
