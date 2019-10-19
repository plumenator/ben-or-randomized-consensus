use crate::message::{Phase, Value};

#[derive(Debug, PartialEq)]
pub(crate) struct Outcome {
    phase: Phase,
    decision: Decision,
}

impl Outcome {
    pub(crate) fn generate(
        init: Value,
        phases: impl Iterator<Item = Phase>,
        step_fn: impl Fn(Phase, Value) -> Decision,
    ) -> impl Iterator<Item = Self> {
        let mut current = Decision::Pending { next: init };
        phases.map(move |phase| {
            let temp = current.clone();
            current = match current.clone() {
                Decision::Done {
                    next,
                    decided: prev_decided,
                } => {
                    let decision = step_fn(phase.next(), next);
                    assert_eq!(prev_decided, decision.decided().expect("has decided"));
                    decision
                }
                Decision::Pending { next } => step_fn(phase.next(), next),
            };
            Outcome {
                phase,
                decision: temp,
            }
        })
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

    #[test]
    fn outcome_generate_works() {
        let step = |phase: Phase, _value| -> Decision {
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
        };

        let mut it = Outcome::generate(Value::Zero, Phase::generate(), step).take(6);
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
