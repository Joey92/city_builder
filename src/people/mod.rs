use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use big_brain::prelude::*;

pub mod actions;
pub mod names;
pub mod scorers;

use crate::{
    building::Home as HomeBuilding,
    work::{AvailableWorkers, Workplaces},
};

use self::{
    actions::{
        buy_stuff::Shopping, drink::Drink, eat::Eat, find_nearest_store::FindClosestStore,
        go_home::GoHome, go_to_destination::GoToDestination, go_work::GoWork, work::Work,
        ActionPlugin,
    },
    names::get_random_name,
    scorers::{
        hungry::Hungry, idling::Idling, inventory_low::InventoryLow, thirsty::Thirsty,
        work::TimeToWork, ScorerPlugin,
    },
};

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Age(pub i8);

#[derive(Component, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Component, PartialEq)]
pub enum CurrentActivity {
    FreeTime,
    Working,
    Sleeping,
}

#[derive(Component)]
pub struct Home(pub Entity);

#[derive(Component)]
pub struct Workplace(pub Entity);

#[derive(Component)]
pub struct Family;

#[derive(Component)]
pub struct Destination(pub Entity);

#[derive(Bundle)]
pub struct PersonBundle {
    pub name: Name,
    pub age: Age,
    pub gender: Gender,
    pub hunger: Hunger,
    pub thirst: Thirst,
    pub cash: Cash,
    pub inventory: Inventory,
    pub state: CurrentActivity,
}

pub trait IncreasingNeed {
    fn increase(&mut self);
}

#[derive(Component)]
pub struct Hunger {
    pub amount: f32,
    pub rate: f32,
}

impl IncreasingNeed for Hunger {
    fn increase(&mut self) {
        let amount = self.amount + self.rate;
        if amount > 100. {
            return;
        }

        self.amount = amount
    }
}

#[derive(Component)]
pub struct Thirst {
    pub amount: f32,
    pub rate: f32,
}

impl IncreasingNeed for Thirst {
    fn increase(&mut self) {
        let amount = self.amount + self.rate;
        if amount > 100. {
            return;
        }

        self.amount = amount
    }
}

#[derive(Component)]
pub struct Cash {
    pub amount: f32,
    pub rate: f32,
}

impl IncreasingNeed for Cash {
    fn increase(&mut self) {
        self.amount += self.rate
    }
}

#[derive(Component)]
pub struct Sleep {
    pub amount: f32,
    pub rate: f32,
}

impl IncreasingNeed for Sleep {
    fn increase(&mut self) {
        self.amount += self.rate
    }
}

fn increase_need<T: Component + IncreasingNeed>(mut needs: Query<&mut T>) {
    for mut need in needs.iter_mut() {
        need.increase()
    }
}

// Scorers are the same as in the thirst example.

#[derive(Component, Debug)]
pub struct Inventory {
    pub food: u32,
    pub drink: u32,
    pub max_stack: u32,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            food: 0,
            drink: 0,
            max_stack: 5,
        }
    }

    pub fn is_full(&self) -> bool {
        self.food == self.max_stack && self.drink == self.max_stack
    }

    pub fn load(&self) -> f32 {
        ((self.food + self.drink) / (self.max_stack * 2)) as f32
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            food: Default::default(),
            drink: Default::default(),
            max_stack: 5,
        }
    }
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                increase_need::<Thirst>,
                increase_need::<Hunger>,
                increase_need::<Cash>,
                increase_need::<Sleep>,
            ),
        )
        .add_plugins(ScorerPlugin)
        .add_plugins(ActionPlugin)
        .insert_resource(EmploymentSearch(Timer::new(
            Duration::from_secs(60),
            TimerMode::Repeating,
        )))
        // whenever a new home is built, a person moves in
        .add_systems(Update, (unemployed_get_jobs_assigned, move_into_new_house));
    }
}

fn get_people_thinker() -> ThinkerBuilder {
    let go_shopping = Steps::build()
        .label("GoToStoreAndShop")
        .step(FindClosestStore)
        .step(GoToDestination)
        .step(Shopping {
            time_to_buy: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        });

    let go_work = Steps::build()
        .label("GoWork")
        .step(GoWork)
        .step(GoToDestination)
        .step(Work {
            shift: Timer::new(Duration::from_secs(60), TimerMode::Once),
        });

    // todo: check if dudde already at home
    let go_home = Steps::build()
        .label("GoHomeAndChill")
        // ...set destination to home...
        .step(GoHome)
        // ...go there...
        .step(GoToDestination);

    // Build the thinker
    Thinker::build()
        .label("PeopleThinker")
        .picker(FirstToScore { threshold: 0.6 })
        .when(Thirsty, Drink)
        .when(Hungry, Eat)
        .when(InventoryLow, go_shopping)
        .when(
            Idling {
                cooldown: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            },
            go_home,
        )
        .when(TimeToWork, go_work)
}

pub const PEOPLE_SHAPE: Circle = Circle::new(1.0);
pub const PEOPLE_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);

fn move_into_new_house(
    mut cmd: Commands,
    house: Query<(Entity, &Transform), Added<HomeBuilding>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (home, location) in house.iter() {
        cmd.spawn((
            Person,
            get_people_thinker(),
            PersonBundle {
                state: CurrentActivity::FreeTime,
                name: Name::new(String::from(get_random_name())),
                age: Age(30),
                gender: Gender::Male,
                hunger: Hunger {
                    amount: 0.,
                    rate: 0.02,
                },
                thirst: Thirst {
                    amount: 0.,
                    rate: 0.03,
                },
                cash: Cash {
                    amount: 100.,
                    rate: 0.1,
                },
                inventory: Inventory::new(),
            },
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(PEOPLE_SHAPE)),
                material: materials.add(PEOPLE_COLOR),
                visibility: Visibility::Visible,
                transform: Transform::from_xyz(
                    location.translation.x,
                    location.translation.y,
                    location.translation.z,
                ),
                ..default()
            },
            Home(home),
        ));
    }
}

#[derive(Resource, Default)]
pub struct EmploymentSearch(pub Timer);

pub fn unemployed_get_jobs_assigned(
    mut cmd: Commands,
    time: Res<Time>,
    mut search_resource: ResMut<EmploymentSearch>,
    jobless: Query<(Entity, &Name), Without<Workplace>>,
    mut workplaces: Query<(
        Entity,
        &mut Workplaces,
        &mut AvailableWorkers,
        Option<&Name>,
    )>,
) {
    if !search_resource.0.tick(time.delta()).finished() {
        // Only assign jobs every x amount of time
        return;
    }

    for (jobseeker_entity, actor_name) in jobless.iter() {
        for (workplace_entity, mut available_jobs, mut available_workers, workplace_name) in
            workplaces.iter_mut()
        {
            if available_jobs.amount == 0 {
                continue;
            }

            cmd.entity(jobseeker_entity)
                .insert((Workplace(workplace_entity), available_jobs.role));

            available_jobs.amount -= 1;
            available_workers.max_workers += 1;

            if available_jobs.amount == 0 {
                cmd.entity(jobseeker_entity).remove::<Workplaces>();
            }

            match workplace_name {
                Some(work_name) => {
                    println!("{:?} found a job at {:?}", actor_name, work_name)
                }
                None => println!("{:?} found a job.", actor_name),
            }
        }
    }
}
