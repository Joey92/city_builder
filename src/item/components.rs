use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemType {
    Food,
    Drink,
}

#[derive(Component)]
pub struct Item(pub ItemType);
