#[derive(Debug, PartialEq)]
struct Message {
    kind: MessageKind,
    phase: u64,
    value: Option<Value>,
}

#[derive(Debug, PartialEq)]
enum Value {
    One,
    Zero,
}

#[derive(Debug, PartialEq)]
enum MessageKind {
    Report,
    Proposal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality() {
        assert_eq!(
            Message {
                kind: MessageKind::Proposal,
                phase: 56,
                value: Some(Value::One)
            },
            Message {
                kind: MessageKind::Proposal,
                phase: 56,
                value: Some(Value::One)
            }
        );
        assert_ne!(
            Message {
                kind: MessageKind::Proposal,
                phase: 56,
                value: Some(Value::Zero)
            },
            Message {
                kind: MessageKind::Report,
                phase: 56,
                value: Some(Value::One)
            }
        );
    }
}
