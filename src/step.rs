use crate::{
    message::{Message, Phase, Value},
    outcome::{Context, Decision},
};

pub(crate) fn correct(
    context: &Context,
    current_phase: Phase,
    current_value: Value,
    num_adversaries: usize,
) -> Decision {
    let Context {
        id,
        senders,
        receiver,
    } = context;
    let num_processes = senders.len();
    assert!(num_processes > num_adversaries);

    // send (R, k, x) to all processes
    eprintln!(
        "Process {}: send (R, {}, {}) to all processes",
        id.0, current_phase.0, current_value
    );
    for sender in senders {
        let report = Message::Report {
            phase: current_phase,
            value: current_value.clone(),
        };
        let _ = sender.send(report.clone()).expect("send");
    }

    // wait for messages of the form (R, k, *) from n - f
    // processes {"*" can be 0 or 1}
    eprintln!(
        "Process {}: wait for messages of the form (R, {}, *) from {}",
        id.0,
        current_phase.0,
        num_processes - num_adversaries
    );
    let (ones, zeros) = read_values(&receiver, num_processes - num_adversaries, |message| {
        eprintln!("Process {}: Received {:?}", id.0, message);
        match &message {
            Message::Report { phase, value } => {
                if phase == &current_phase {
                    Some(Some(value.clone()))
                } else {
                    eprintln!("Process {}: dropped {:?}", id.0, message);
                    None
                }
            }
            Message::Proposal { phase, value: _ } => {
                if phase >= &current_phase {
                    panic!("Process {}: skipped {:?}", id.0, message);
                } else {
                    eprintln!("Process {}: dropped {:?}", id.0, message);
                }
                None
            }
        }
    });
    let mut potentials = if ones.len() > zeros.len() {
        ones
    } else {
        zeros
    };

    // if received more than n/2 (R, k, v) with the same v
    eprintln!(
        "Process {}: if received more than {} (R, {}, v) with the same v",
        id.0,
        num_processes / 2,
        current_phase.0
    );
    if potentials.len() > num_processes / 2 {
        let potential = potentials.pop().expect("at least one");
        // then send (P, k, v) to all processes
        eprintln!(
            "Process {}: then send (P, {}, {}) to all processes",
            id.0, current_phase.0, potential
        );
        for sender in senders {
            let proposal = Message::Proposal {
                phase: current_phase,
                value: Some(potential.clone()),
            };
            let _ = sender.send(proposal.clone()).expect("send");
        }
    } else {
        // else send (P, k, ?) to all processes
        eprintln!(
            "Process {}: else send (P, {}, ?) to all processes",
            id.0, current_phase.0
        );
        for sender in senders {
            let proposal = Message::Proposal {
                phase: current_phase,
                value: None,
            };
            let _ = sender.send(proposal.clone()).expect("send");
        }
    }

    // wait for messages of the form (P, k, *) from n - f
    // processes {"*" can be 0 or 1}
    eprintln!(
        "Process {}: wait for messages of the form (P, {}, *) from {}",
        id.0,
        current_phase.0,
        num_processes - num_adversaries
    );
    let (ones, zeros) = read_values(&receiver, num_processes - num_adversaries, |message| {
        eprintln!("Process {}: Received {:?}", id.0, message);
        match &message {
            Message::Proposal { phase, value } => {
                if phase == &current_phase {
                    Some(value.clone())
                } else {
                    eprintln!("Process {}: dropped {:?}", id.0, message);
                    None
                }
            }
            Message::Report { phase, value: _ } => {
                if phase > &current_phase {
                    panic!("Process {}: skipped {:?}", id.0, message);
                } else {
                    eprintln!("Process {}: dropped {:?}", id.0, message);
                }
                None
            }
        }
    });
    let mut potentials = if ones.len() > zeros.len() {
        ones
    } else {
        zeros
    };

    let num_potentials = potentials.len();
    let potential = potentials.pop();

    // if at least one (P, k, v) with v != ?
    eprintln!(
        "Process {}: if at least one (P, {}, v) with v != ?",
        id.0, current_phase.0
    );
    let next = if let &Some(ref value) = &potential {
        // TODO: this is less general because we pick the majority

        // then x <- v
        eprintln!("Process {}: then x <- {}", id.0, value);
        value.clone()
    } else if rand::random::<bool>() {
        // else x <- 1 randomly {query r.n.g}
        eprintln!("Process {}: else x <- 1 randomly", id.0);
        Value::One
    } else {
        // else x <- 0 randomly {query r.n.g}
        eprintln!("Process {}: else x <- 0 randomly", id.0);
        Value::Zero
    };

    // if received at least f + 1 (P, k, v) with the same v != ?
    eprintln!(
        "Process {}: if received at least f + 1 (P, {}, v) with the same v != ?",
        id.0, current_phase.0
    );
    if num_potentials > num_adversaries {
        // then decide(v)
        eprintln!(
            "Process {}: then decide({})",
            id.0,
            potential.clone().expect("exists")
        );
        Decision::Done {
            next,
            decided: potential.expect("exists"),
        }
    } else {
        // else send (P, k, ?) to all processes
        eprintln!(
            "Process {}: else send (P, {}, ?) to all processes",
            id.0, current_phase.0
        );
        Decision::Pending { next }
    }
}

fn read_values(
    receiver: &std::sync::mpsc::Receiver<Message>,
    take: usize,
    filter_map_fn: impl Fn(Message) -> Option<Option<Value>>,
) -> (Vec<Value>, Vec<Value>) {
    let mut ones = vec![];
    let mut zeros = vec![];
    let mut count: usize = 0;
    while count < take {
        let message = receiver.recv().expect("recv");
        if let Some(value) = filter_map_fn(message) {
            count = count + 1;
            if let Some(Value::One) = value {
                ones.push(Value::One);
            } else if let Some(Value::Zero) = value {
                zeros.push(Value::Zero);
            }
        }
    }
    (ones, zeros)
}
