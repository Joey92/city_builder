use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::{Inventory, Thirst};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Drink;

pub fn drink_action_system(
    mut thirst: Query<(&Name, &mut Thirst, &mut Inventory)>,
    mut query: Query<(&Actor, &mut ActionState, &Drink, &ActionSpan)>,
) {
    // Loop through all actions, just like you'd loop over all entities in any other query.
    for (Actor(actor), mut state, _, span) in query.iter_mut() {
        let _guard = span.span().enter();

        // Look up the actor's position and thirst from the Actor component in the action entity.
        let (actor_name, mut thirst, mut inventory) =
            thirst.get_mut(*actor).expect("actor has no thirst");

        match *state {
            ActionState::Requested => {
                // We'll start drinking as soon as we're requested to do so.
                println!("{:?} wants to drink water...", actor_name);
                if inventory.drink == 0 {
                    println!("{:?} has no more water in inventory", actor_name);
                    *state = ActionState::Failure;
                    continue;
                }
                inventory.drink -= 1;
                *state = ActionState::Executing;
            }
            ActionState::Executing | ActionState::Cancelled => {
                thirst.amount -= 0.3;

                if thirst.amount <= 0. {
                    println!("{:?} drank enough water", actor_name);
                    *state = ActionState::Success;
                }
            }
            _ => {}
        }
    }
}
