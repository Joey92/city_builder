use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

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

pub const BUILDING_COLOR: Color = Color::CYAN;

pub fn leftclick_to_build(
    mut cmd: Commands,
    target: Res<MouseTarget>,
    buttons: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            Home,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0))),
                material: materials.add(BUILDING_COLOR),
                visibility: Visibility::Visible,
                transform: Transform::from_xyz(target.0.x, target.0.y, 0.),
                ..default()
            }, // Name::new("the Residence"),
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
