extern crate rand;

mod message;
mod outcome;
mod process;
mod step;

use crate::{
    message::Value,
    outcome::Outcome,
    process::{Id, Process},
};

pub fn simulate(
    num_processes: usize,
    num_adversaries: usize,
    num_zeros: usize,
) -> impl Iterator<Item = (Id, Outcome)> {
    let mut senders = vec![];
    let mut receivers = vec![];
    for _ in 0..num_processes {
        let (sender, receiver) = std::sync::mpsc::channel();
        senders.push(sender);
        receivers.push(receiver);
    }
    let mut processes = vec![];
    for (i, receiver) in receivers.into_iter().enumerate() {
        processes.push(Process {
            id: Id(i),
            senders: senders.clone(),
            receiver: receiver,
        })
    }
    let (sender, receiver) = std::sync::mpsc::channel();
    for process in processes {
        let sender = sender.clone();
        let init = if process.id.0 < num_zeros {
            Value::Zero
        } else {
            Value::One
        };
        let _ = std::thread::spawn(move || {
            for (id, outcome) in process.run(init, step::correct, num_adversaries) {
                sender.send((id, outcome)).expect("send");
            }
        });
    }
    receiver.into_iter()
}
