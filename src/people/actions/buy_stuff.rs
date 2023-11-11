use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    building::store::Store,
    item::components::ItemType,
    people::{Cash, Destination, Inventory},
};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Shopping {
    pub time_to_buy: Timer,
}

pub fn buy_stuff_action_system(
    time: Res<Time>,
    mut shoppers: Query<(&Name, &mut Cash, &mut Inventory, &Destination)>,
    mut stores: Query<(&Transform, &mut Store)>,
    mut query: Query<(&Actor, &mut ActionState, &mut Shopping, &ActionSpan)>,
) {
    for (Actor(actor), mut state, mut shopping, span) in query.iter_mut() {
        let _guard = span.span().enter();

        let (actor_name, mut cash, mut inventory, Destination(store_entity)) = shoppers
            .get_mut(*actor)
            .expect("actor for action not found. Maybe it's missing some components we need?");

        match *state {
            ActionState::Requested => {
                // We'll start drinking as soon as we're requested to do so.
                println!("{:?} wants to buy stuff..", actor_name);
                *state = ActionState::Executing;
            }
            ActionState::Executing | ActionState::Cancelled => {
                if !shopping.time_to_buy.tick(time.delta()).finished() {
                    continue;
                }

                let (_, mut store) = stores
                    .get_mut(*store_entity)
                    .expect("Destination is not a store");

                if inventory.food < inventory.max_stack {
                    let stock_opt = store.stock.get_mut(&ItemType::Food);

                    match stock_opt {
                        Some(stock) => {
                            let price = *stock.price();
                            if cash.amount < price {
                                println!(
                                    "{:?} does not have enough money to buy anything anymore",
                                    actor_name
                                );
                                *state = ActionState::Failure;
                                continue;
                            }

                            if !stock.take(1) {
                                println!(
                                    "{:?} can't buy food. Stock seems to be empty!",
                                    actor_name
                                );
                                *state = ActionState::Failure;
                                continue;
                            }

                            cash.amount -= price;
                            inventory.food += 1;
                            println!("{:?} bought some food", actor_name);
                        }
                        None => {
                            println!(
                                "{:?} can't buy food because the store does not sell this.",
                                actor_name
                            );
                            *state = ActionState::Failure;
                            continue;
                        }
                    }
                }

                if inventory.drink < inventory.max_stack {
                    let stock_opt = store.stock.get_mut(&ItemType::Drink);

                    match stock_opt {
                        Some(stock) => {
                            let price = *stock.price();
                            if cash.amount < price {
                                println!("{:?} can't buy a drink. No more money!", actor_name);
                                *state = ActionState::Failure;
                                continue;
                            }

                            if !stock.take(1) {
                                println!(
                                    "{:?} can't buy a drink. Stock seems to be empty!",
                                    actor_name
                                );
                                *state = ActionState::Failure;
                                continue;
                            }

                            cash.amount -= price;
                            inventory.drink += 1;
                            println!("{:?} bought a drink", actor_name);
                        }
                        None => {
                            println!(
                                "{:?} can't buy drinks because the store does not sell this.",
                                actor_name
                            );
                            *state = ActionState::Failure;
                            continue;
                        }
                    }
                }

                if inventory.is_full() {
                    println!("{:?} is done with shopping.", actor_name);
                    *state = ActionState::Success;
                }
            }
            _ => {}
        }
    }
}
