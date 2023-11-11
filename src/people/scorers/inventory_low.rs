use bevy::prelude::*;
use big_brain::prelude::*;

use crate::people::Inventory;

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct InventoryLow;

pub fn inventory_scorer_system(
    inventories: Query<Ref<Inventory>>,
    mut query: Query<(&Actor, &mut Score), With<InventoryLow>>,
) {
    for (Actor(actor), mut score) in query.iter_mut() {
        if let Ok(inventory) = inventories.get(*actor) {
            if inventory.is_changed() || inventory.is_added() || score.is_added() {
                score.set(1. - (inventory.food + inventory.drink) as f32 * 0.1);
            }
        }
    }
}
