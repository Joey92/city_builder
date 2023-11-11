use bevy::prelude::Plugin;

pub mod components;
pub mod stock;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
