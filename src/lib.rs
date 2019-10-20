extern crate rand;

mod message;
mod outcome;
mod process;
mod step;
mod transport;

use crate::{
    message::Value,
    outcome::Outcome,
    process::{Id, Process},
    transport::Transport,
};

pub use crate::step::Behavior;

pub fn simulate(
    num_processes: usize,
    num_zeros: usize,
    num_adversaries: usize,
    adversial_behavior: Behavior,
) -> impl Iterator<Item = (Id, Outcome)> {
    let mut processes = vec![];
    for (i, transport) in Transport::new(num_processes).into_iter().enumerate() {
        processes.push(Process {
            id: Id(i),
            transport,
        })
    }
    let (sender, receiver) = std::sync::mpsc::channel();
    for process in processes {
        let sender = sender.clone();
        assert!(num_zeros <= num_processes);
        let init = if process.id.0 < num_zeros {
            Value::Zero
        } else {
            Value::One
        };
        let step_fn = if process.id.0 < num_adversaries {
            adversial_behavior.step_fn()
        } else {
            Behavior::Correct.step_fn()
        };
        let _ = std::thread::spawn(move || {
            for (id, outcome) in process.run(init, step_fn, num_adversaries) {
                sender.send((id, outcome)).expect("send outcome");
            }
        });
    }
    receiver.into_iter()
}
