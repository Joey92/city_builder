use bevy::prelude::*;

trait Workplace {
    fn do_work();
}

#[derive(Component, PartialEq, Clone, Copy)]
pub enum Role {
    ShopKeeper,
    Police,
    Logistic,
}

#[derive(Component)]
pub struct Workplaces {
    pub amount: u32,
    pub role: Role,
}

#[derive(Component)]
pub struct AvailableWorkers {
    pub current_workers: u32,
    pub max_workers: u32,
}

#[derive(Bundle)]
pub struct WorkplaceBundle {
    pub workplaces: Workplaces,
    pub workers_available: AvailableWorkers,
}

impl WorkplaceBundle {
    pub fn new(jobs: u32, role: Role) -> Self {
        Self {
            workplaces: Workplaces { amount: jobs, role },
            workers_available: AvailableWorkers {
                current_workers: 0,
                max_workers: 0,
            },
        }
    }
}
