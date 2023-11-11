use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;

use crate::{item::{components::ItemType, stock::Stock}, work::AvailableWorkers};

#[derive(Component)]
pub struct Store {
    pub stock: HashMap<ItemType, Stock>,
    pub restock_rate: Timer,
}

impl Default for Store {
    fn default() -> Self {
        Self {
            stock: Default::default(),
            restock_rate: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

#[derive(Bundle)]
pub struct StoreBundle {
    pub store: Store,
    pub name: Name,
}

pub fn store_workers_replenish_stock(
    time: Res<Time>,
    mut stores: Query<(&AvailableWorkers, &mut Store), With<Store>>,
) {
    for (work, mut store) in stores.iter_mut() {
        if work.current_workers == 0 {
            continue;
        }

        if !store.restock_rate.tick(time.delta()).finished() {
            continue;
        }

        if let Some(stock) = store.stock.get_mut(&ItemType::Drink) {
            stock.replenish(work.current_workers);
        }

        if let Some(stock) = store.stock.get_mut(&ItemType::Food) {
            stock.replenish(work.current_workers);
        }
    }
}
