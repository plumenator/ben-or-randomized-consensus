use std::sync::mpsc::{Receiver, Sender};

use crate::{
    message::{Message, Phase, Value},
    outcome::{Decision, Outcome},
};

struct Process {
    id: Id,
    senders: Vec<Sender<Message>>,
    receiver: Receiver<Message>,
}

#[derive(Clone)]
struct Id(u64);

impl Process {
    fn run(
        self,
        init: Value,
        step_fn: impl Fn(Phase, Value) -> Decision,
    ) -> impl Iterator<Item = (Id, Outcome)> {
        Outcome::generate(init, Phase::generate(), step_fn)
            .map(move |outcome| (self.id.clone(), outcome))
    }
}
