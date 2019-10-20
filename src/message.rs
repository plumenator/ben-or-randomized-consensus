use std::{
    convert::{TryFrom, TryInto},
    fmt, iter,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Proposal { phase: Phase, value: Option<Value> },
    Report { phase: Phase, value: Value },
}

impl From<Message> for Vec<u8> {
    fn from(message: Message) -> Self {
        match message {
            Message::Proposal { phase, value: None } => {
                let mut bytes = vec![0];
                bytes.extend(&phase.0.to_be_bytes());
                bytes
            }
            Message::Proposal {
                phase,
                value: Some(value),
            } => {
                let mut bytes = vec![1];
                bytes.extend(&phase.0.to_be_bytes());
                bytes.push(value.into());
                bytes
            }
            Message::Report { phase, value } => {
                let mut bytes = vec![2];
                bytes.extend(&phase.0.to_be_bytes());
                bytes.push(value.into());
                bytes
            }
        }
    }
}

impl TryFrom<Vec<u8>> for Message {
    type Error = &'static str;

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        let num_bytes = bytes.len();
        match num_bytes {
            9 => {
                if bytes[0] != 0 {
                    Err("not an undecided proposal")
                } else {
                    let phase = Phase(u64::from_be_bytes(
                        bytes[1..9].try_into().expect("array of 8"),
                    ));
                    Ok(Message::Proposal { phase, value: None })
                }
            }
            10 => {
                if bytes[0] == 1 {
                    let phase = Phase(u64::from_be_bytes(
                        bytes[1..9].try_into().map_err(|_| "not an array of 8")?,
                    ));
                    Ok(Message::Proposal {
                        phase,
                        value: Some(bytes[9].try_into()?),
                    })
                } else if bytes[0] == 2 {
                    let phase = Phase(u64::from_be_bytes(
                        bytes[1..9].try_into().map_err(|_| "not an array of 8")?,
                    ));
                    Ok(Message::Report {
                        phase,
                        value: bytes[9].try_into()?,
                    })
                } else {
                    Err("not a decided proposal or report")
                }
            }
            _ => Err("illegal length"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Phase(pub(crate) u64);

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
pub enum Value {
    One,
    Zero,
}

impl From<Value> for u8 {
    fn from(value: Value) -> u8 {
        match value {
            Value::One => 1,
            Value::Zero => 0,
        }
    }
}

impl TryFrom<u8> for Value {
    type Error = &'static str;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        match byte {
            1 => Ok(Value::One),
            0 => Ok(Value::Zero),
            _ => Err("not a value"),
        }
    }
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

    #[test]
    fn serialization() {
        let undecided_proposal_bytes: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 0, 56];
        let undecided_proposal = Message::Proposal {
            phase: Phase(56),
            value: None,
        };
        assert_eq!(
            undecided_proposal_bytes,
            Vec::<u8>::from(undecided_proposal.clone())
        );
        assert_eq!(
            Ok(undecided_proposal),
            Message::try_from(undecided_proposal_bytes)
        );

        let decided_proposal_bytes: Vec<u8> = vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 1];
        let decided_proposal = Message::Proposal {
            phase: Phase(256),
            value: Some(Value::One),
        };
        assert_eq!(
            decided_proposal_bytes,
            Vec::<u8>::from(decided_proposal.clone())
        );
        assert_eq!(
            Ok(decided_proposal),
            Message::try_from(decided_proposal_bytes)
        );

        let report_bytes: Vec<u8> = vec![2, 0, 0, 0, 0, 0, 0, 0, 56, 0];
        let report = Message::Report {
            phase: Phase(56),
            value: Value::Zero,
        };
        assert_eq!(report_bytes, Vec::<u8>::from(report.clone()));
        assert_eq!(Ok(report), Message::try_from(report_bytes));
    }
}
