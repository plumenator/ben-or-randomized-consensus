use std::fmt;

use crate::{
    message::{Phase, Value},
    outcome::{self, Context, Decision, Outcome},
    transport::Transport,
};

pub(crate) struct Process {
    pub(crate) id: Id,
    pub(crate) transport: Transport,
}

#[derive(Clone)]
pub struct Id(pub(crate) usize);

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Process {
    pub(crate) fn run(
        self,
        init: Value,
        step_fn: impl Fn(&Context, Phase, Value, usize) -> Decision,
        num_adversaries: usize,
    ) -> impl Iterator<Item = (Id, Outcome)> {
        let Self { id, transport } = self;
        Outcome::generate(
            init,
            Phase::generate(),
            step_fn,
            Context {
                id: outcome::ProcessId(id.0),
                transport,
            },
            num_adversaries,
        )
        .map(move |outcome| (id.clone(), outcome))
    }
}
