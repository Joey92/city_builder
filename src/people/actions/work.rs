use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{people::Workplace, work::AvailableWorkers};

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Work {
    pub shift: Timer,
}

pub fn work_action_system(
    time: Res<Time>,
    workers: Query<(&Name, &Workplace)>,
    mut workplaces: Query<(&mut AvailableWorkers, Option<&Name>), Without<Workplace>>,
    mut query: Query<(&Actor, &mut ActionState, &mut Work, &ActionSpan)>,
) {
    for (Actor(actor), mut state, mut work, span) in query.iter_mut() {
        let _guard = span.span().enter();

        let (actor_name, workplace) = workers
            .get(*actor)
            .expect("actor for action not found. Maybe it's missing some components we need?");

        let (mut available_workers, workplace_name) = workplaces
            .get_mut(workplace.0)
            .expect("Workplace not found");

        match *state {
            ActionState::Requested => {
                match workplace_name {
                    Some(name) => println!("{:?} starting work shift at {:?}..", actor_name, name),
                    None => println!("{:?} starting work shift..", actor_name),
                }

                available_workers.current_workers += 1;
                *state = ActionState::Executing;
            }
            ActionState::Executing | ActionState::Cancelled => {
                if !work.shift.tick(time.delta()).finished() {
                    continue;
                }

                work.shift.reset();
                println!("{:?} is done working.", actor_name);
                available_workers.current_workers -= 1;
                *state = ActionState::Success;
            }
            _ => {}
        }
    }
}
