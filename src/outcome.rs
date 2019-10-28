use std::fmt;

use crate::{
    message::{Phase, Value},
    transport::Transport,
};

pub(crate) struct Context {
    pub(crate) id: ProcessId,
    pub(crate) transport: Box<dyn Transport>,
}

#[derive(Clone)]
pub(crate) struct ProcessId(pub(crate) usize);

#[derive(Debug, PartialEq)]
pub struct Outcome {
    phase: Phase,
    decision: Decision,
}

impl Outcome {
    pub(crate) fn generate(
        init: Value,
        phases: impl Iterator<Item = Phase>,
        step_fn: impl Fn(&Context, Phase, Value, usize) -> Decision,
        context: Context,
        num_adversaries: usize,
    ) -> impl Iterator<Item = Self> {
        let mut current = Decision::Pending { next: init };
        phases.map(move |phase| {
            let temp = current.clone();
            current = match current.clone() {
                Decision::Done {
                    next,
                    decided: prev_decided,
                } => {
                    let decision = step_fn(&context, phase.next(), next, num_adversaries);
                    if let Some(decided) = decision.decided() {
                        assert_eq!(prev_decided, decided);
                    }
                    decision
                }
                Decision::Pending { next } => {
                    step_fn(&context, phase.next(), next, num_adversaries)
                }
            };
            Outcome {
                phase,
                decision: temp,
            }
        })
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Outcome { phase, decision } = self;
        match decision {
            Decision::Done { next, decided } => write!(
                f,
                "(Phase: {}, Next: {}, Decide: {})",
                phase.0, next, decided
            ),
            Decision::Pending { next } => write!(f, "(Phase: {}, Next: {})", phase.0, next),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Decision {
    Done { next: Value, decided: Value },
    Pending { next: Value },
}

impl Decision {
    fn decided(&self) -> Option<Value> {
        match &self {
            Decision::Done { next: _, decided } => Some(decided.clone()),
            Decision::Pending { next: _ } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::transport::MessageChannel;

    fn step_fn(_context: &Context, phase: Phase, _value: Value, _: usize) -> Decision {
        let next = if phase.0 % 2 == 1 {
            Value::Zero
        } else {
            Value::One
        };
        if phase.0 >= 4 {
            Decision::Done {
                next,
                decided: Value::Zero,
            }
        } else {
            Decision::Pending { next }
        }
    }

    #[test]
    fn outcome_generate_works() {
        let mut it = Outcome::generate(
            Value::Zero,
            Phase::generate(),
            step_fn,
            Context {
                id: ProcessId(0),
                transport: MessageChannel::new(1).remove(0),
            },
            0,
        )
        .take(6);
        assert_eq!(
            it.next(),
            Some(Outcome {
                phase: Phase(0),
                decision: Decision::Pending { next: Value::Zero }
            })
        );
        assert_eq!(
            it.next(),
            Some(Outcome {
                phase: Phase(1),
                decision: Decision::Pending { next: Value::Zero }
            })
        );
        assert_eq!(
            it.next(),
            Some(Outcome {
                phase: Phase(2),
                decision: Decision::Pending { next: Value::One }
            })
        );
        assert_eq!(
            it.next(),
            Some(Outcome {
                phase: Phase(3),
                decision: Decision::Pending { next: Value::Zero }
            })
        );
        assert_eq!(
            it.next(),
            Some(Outcome {
                phase: Phase(4),
                decision: Decision::Done {
                    next: Value::One,
                    decided: Value::Zero
                }
            })
        );
        assert_eq!(
            it.next(),
            Some(Outcome {
                phase: Phase(5),
                decision: Decision::Done {
                    next: Value::Zero,
                    decided: Value::Zero
                }
            })
        );
        assert_eq!(it.next(), None);
    }
}
