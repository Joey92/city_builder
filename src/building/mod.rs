use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    item::components::ItemType,
    work::{Role, Workplaces},
    MouseTarget,
};

use self::store::{store_workers_replenish_stock, Store};

pub mod store;

#[derive(Component)]
pub struct Building {
    pub entrance: Transform,
}

#[derive(Component)]
pub struct Home;

#[derive(Bundle)]
pub struct BuildingBundle {
    sprite: SpriteBundle,
}

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                leftclick_to_build,
                store_workers_replenish_stock,
                announce_stock,
            ),
        );
    }
}

fn announce_stock(stores: Query<Ref<Store>>) {
    for store in stores.iter() {
        if store.is_changed() {
            println!(
                "Food: {:?}, Drink: {:?}",
                store
                    .stock
                    .get(&ItemType::Food)
                    .map_or_else(|| 0, |s| s.amount),
                store
                    .stock
                    .get(&ItemType::Drink)
                    .map_or_else(|| 0, |s| s.amount)
            )
        }
    }
}

pub const BUILDING_SHAPE: shapes::Rectangle = shapes::Rectangle {
    extents: Vec2 { x: 10., y: 10. },
    origin: shapes::RectangleOrigin::Center,
};

pub fn leftclick_to_build(
    mut cmd: Commands,
    target: Res<MouseTarget>,
    buttons: Res<Input<MouseButton>>,
) {
    // if buttons.just_pressed(MouseButton::Left) {
    //     // Left button was pressed
    // }
    if buttons.just_released(MouseButton::Left) {
        // Left Button was released
        cmd.spawn((
            Building {
                entrance: Transform::from_xyz(target.0.x, target.0.y, 0.),
            },
            Workplaces {
                amount: 3,
                role: Role::ShopKeeper,
            },
            ShapeBundle {
                path: GeometryBuilder::build_as(&BUILDING_SHAPE),
                transform: Transform::from_xyz(target.0.x, target.0.y, 0.),
                ..default()
            },
            Fill::color(Color::CYAN),
            Stroke::new(Color::BLACK, 5.0),
            Home,
            // Name::new("the Residence"),
        ));
    }
    // if buttons.pressed(MouseButton::Right) {
    //     // Right Button is being held down
    // }
    // // we can check multiple at once with `.any_*`
    // if buttons.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
    //     // Either the left or the right button was just pressed
    // }
}
