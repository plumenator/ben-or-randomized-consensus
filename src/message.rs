use std::{fmt, iter};

#[derive(Debug, PartialEq)]
pub(crate) enum Message {
    Proposal { phase: Phase, value: Option<Value> },
    Report { phase: Phase, value: Value },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Phase(pub(crate) u64);

impl Phase {
    pub(crate) fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub(crate) fn generate() -> impl Iterator<Item = Self> {
        count_from(0).map(Phase)
    }
}

fn count_from(init: u64) -> impl Iterator<Item = u64> {
    let mut current = init;
    iter::repeat_with(move || {
        let temp = current;
        current = current + 1;
        temp
    })
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    One,
    Zero,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::One => write!(f, "1",),
            Value::Zero => write!(f, "0",),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(
            Message::Proposal {
                phase: Phase(56),
                value: Some(Value::One)
            },
            Message::Proposal {
                phase: Phase(56),
                value: Some(Value::One)
            }
        );
        assert_eq!(
            Message::Report {
                phase: Phase(56),
                value: Value::One
            },
            Message::Report {
                phase: Phase(56),
                value: Value::One
            }
        );
        assert_ne!(
            Message::Proposal {
                phase: Phase(56),
                value: Some(Value::Zero)
            },
            Message::Report {
                phase: Phase(56),
                value: Value::One
            }
        );
    }

    #[test]
    fn count_from_works() {
        let mut it = count_from(0).take(3);
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn phase_generate_works() {
        let mut it = Phase::generate().take(3);
        assert_eq!(it.next(), Some(Phase(0)));
        assert_eq!(it.next(), Some(Phase(1)));
        assert_eq!(it.next(), Some(Phase(2)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn phase_next_works() {
        assert_eq!(Phase(0).next(), Phase(1));
        assert_eq!(Phase(7).next(), Phase(8));
    }
}
