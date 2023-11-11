use std::time::Duration;

use bevy::window::PrimaryWindow;
use bevy::{prelude::*, DefaultPlugins};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;
use big_brain::BigBrainPlugin;
use building::store::{Store, StoreBundle};
use item::components::ItemType;
use item::stock::Stock;
use work::WorkplaceBundle;

mod building;
mod item;
mod people;
mod work;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<MouseTarget>()
        .add_plugins(ShapePlugin)
        .add_plugins(BigBrainPlugin::new(PreUpdate))
        .add_systems(PreUpdate, cursor_system)
        .add_systems(Startup, startup_app)
        .add_systems(Update, camera)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(building::BuildingPlugin)
        .add_plugins(people::PeoplePlugin)
        .run();
}

// make people go between store and home -> done
// make people get hungry / thirsty and go to store to reset needs -> done
// store creates items over time that restore needs. If out of items it can't restore needs. Restoring needs immediately -> done
// people use cash to purchase items at the store -> done
// transfer items to peoples inventory instead of restoring needs. -> done
// items in inventory are used to restore needs on demand -> done
// limit inventory space and create external inventory in houses
// people buy items in store and bring it into their houses
// restoring needs:
// - check inventory to use items
// - check hous einventory to get items to own inventory
// - go to store to get items
// Make people work for cash
// People who are satisfied and not in need of cash can have free time

fn startup_app(mut cmd: Commands) {
    cmd.spawn((Camera2dBundle::default(), MainCamera));

    let building_shape = shapes::Rectangle {
        extents: Vec2 { x: 10., y: 10. },
        ..shapes::Rectangle::default()
    };

    let mut store = Store {
        stock: Default::default(),
        restock_rate: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    };

    store
        .stock
        .insert(ItemType::Food, Stock::new_for_sale(500, 500, 1.5));
    store
        .stock
        .insert(ItemType::Drink, Stock::new_for_sale(500, 500, 1.5));

    cmd.spawn((
        building::Building {
            entrance: Transform::from_xyz(30., 30., 0.),
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&building_shape),
            transform: Transform::from_xyz(30., 30., 0.),
            ..default()
        },
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 5.0),
        StoreBundle {
            name: Name::new("the Supermarket"),
            store: store,
        },
        WorkplaceBundle::new(3, work::Role::ShopKeeper),
    ));
}

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn camera(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.y += 1.
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.y -= 1.
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += 1.
        }

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= 1.
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

#[derive(Resource)]
pub struct MouseTarget(pub Vec3);

impl Default for MouseTarget {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Component)]
pub struct MainCamera;

fn cursor_system(
    mut target: ResMut<MouseTarget>,
    // need to get window dimensions
    windows: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.get_single().expect("No primary window");

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        target.0.x = world_position.x;
        target.0.y = world_position.y;
    }
}
