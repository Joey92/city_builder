use bevy::prelude::*;

#[derive(Component)]
pub struct Stock {
    pub(crate) amount: u32,
    pub(crate) max: u32,
    pub(crate) price: f32,
}

impl Stock {
    pub fn new(starting_amount: u32, max_amount: u32) -> Self {
        Self {
            amount: starting_amount,
            max: max_amount,
            price: 0.,
        }
    }

    pub fn new_for_sale(starting_amount: u32, max_amount: u32, price: f32) -> Self {
        Self {
            amount: starting_amount,
            max: max_amount,
            price: price,
        }
    }

    pub fn take(&mut self, amount: u32) -> bool {
        if self.amount >= amount {
            self.amount -= amount;
            return true;
        }

        return false;
    }

    pub fn price(&self) -> &f32 {
        return &self.price;
    }

    pub fn replenish(&mut self, amount: u32) {
        if self.amount >= self.max {
            return;
        }
        self.amount += amount;
    }
}
