use std::{fmt, str::FromStr};

use crate::{
    message::{Message, Phase, Value},
    outcome::{Context, Decision},
    transport::Transport,
};

pub enum Behavior {
    Correct,
    Crashes,
    SendsInvalidMessages,
    StopsExecuting,
    RandomlyAdversial,
}

impl Behavior {
    pub(crate) fn step_fn(&self) -> impl Fn(&Context, Phase, Value, usize) -> Decision {
        match self {
            Behavior::Correct => correct,
            Behavior::Crashes => randomly_crashes,
            Behavior::SendsInvalidMessages => randomly_sends_invalid_messages,
            Behavior::StopsExecuting => randomly_stops_executing,
            Behavior::RandomlyAdversial => {
                use rand::seq::SliceRandom;
                [
                    randomly_crashes,
                    randomly_sends_invalid_messages,
                    randomly_stops_executing,
                ]
                .choose(&mut rand::thread_rng())
                .expect("choose")
                .clone()
            }
        }
    }
}

impl fmt::Display for Behavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Behavior::Correct => write!(f, "correct",),
            Behavior::Crashes => write!(f, "crashes",),
            Behavior::SendsInvalidMessages => write!(f, "sends_invalid_messages",),
            Behavior::StopsExecuting => write!(f, "stops_executing",),
            Behavior::RandomlyAdversial => write!(f, "randomly_adversial",),
        }
    }
}

impl FromStr for Behavior {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "correct" => Ok(Behavior::Correct),
            "crashes" => Ok(Behavior::Crashes),
            "sends_invalid_messages" => Ok(Behavior::SendsInvalidMessages),
            "stops_executing" => Ok(Behavior::StopsExecuting),
            "randomly_adversial" => Ok(Behavior::RandomlyAdversial),
            _ => Err(String::from("invalid behavior string")),
        }
    }
}

fn correct(
    context: &Context,
    current_phase: Phase,
    current_value: Value,
    num_adversaries: usize,
) -> Decision {
    let Context { id, transport } = context;
    let num_processes = transport.num_senders();
    assert!(num_processes > num_adversaries);

    // send (R, k, x) to all processes
    eprintln!(
        "Process {}: send (R, {}, {}) to all processes",
        id.0, current_phase.0, current_value
    );
    transport.send(Message::Report {
        phase: current_phase,
        value: current_value.clone(),
    });

    // wait for messages of the form (R, k, *) from n - f
    // processes {"*" can be 0 or 1}
    eprintln!(
        "Process {}: wait for messages of the form (R, {}, *) from {}",
        id.0,
        current_phase.0,
        num_processes - num_adversaries
    );
    let (ones, zeros) = read_values(&transport, num_processes - num_adversaries, |message| {
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
                    transport.send_to_self(message.clone());
                    eprintln!("Process {}: skipped {:?}", id.0, message);
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
        transport.send(Message::Proposal {
            phase: current_phase,
            value: Some(potential.clone()),
        });
    } else {
        // else send (P, k, ?) to all processes
        eprintln!(
            "Process {}: else send (P, {}, ?) to all processes",
            id.0, current_phase.0
        );
        transport.send(Message::Proposal {
            phase: current_phase,
            value: None,
        });
    }

    // wait for messages of the form (P, k, *) from n - f
    // processes {"*" can be 0 or 1}
    eprintln!(
        "Process {}: wait for messages of the form (P, {}, *) from {}",
        id.0,
        current_phase.0,
        num_processes - num_adversaries
    );
    let (ones, zeros) = read_values(&transport, num_processes - num_adversaries, |message| {
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
                    transport.send_to_self(message.clone());
                    eprintln!("Process {}: skipped {:?}", id.0, message);
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
    transport: &Transport,
    take: usize,
    filter_map_fn: impl Fn(Message) -> Option<Option<Value>>,
) -> (Vec<Value>, Vec<Value>) {
    let mut ones = vec![];
    let mut zeros = vec![];
    let mut count: usize = 0;
    while count < take {
        let message = transport.receive();
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

fn randomly_crashes(
    context: &Context,
    current_phase: Phase,
    current_value: Value,
    num_adversaries: usize,
) -> Decision {
    if rand::random::<u64>() % (current_phase.0 + 2) == 0 {
        panic!("Process {}: Crashing", context.id.0)
    } else {
        correct(context, current_phase, current_value, num_adversaries)
    }
}

fn randomly_sends_invalid_messages(
    context: &Context,
    current_phase: Phase,
    current_value: Value,
    num_adversaries: usize,
) -> Decision {
    if rand::random::<bool>() {
        context.transport.send(if rand::random::<bool>() {
            Message::Proposal {
                phase: current_phase,
                value: if rand::random::<bool>() {
                    Some(current_value.clone())
                } else {
                    None
                },
            }
        } else {
            Message::Report {
                phase: current_phase,
                value: current_value.clone(),
            }
        });
        eprintln!("Process {}: Sent random messages", context.id.0);
        Decision::Pending {
            next: current_value,
        }
    } else {
        correct(context, current_phase, current_value, num_adversaries)
    }
}

fn randomly_stops_executing(
    context: &Context,
    current_phase: Phase,
    current_value: Value,
    num_adversaries: usize,
) -> Decision {
    if rand::random::<bool>() {
        eprintln!("Process {}: Stopped executing", context.id.0);
        Decision::Pending {
            next: current_value,
        }
    } else {
        correct(context, current_phase, current_value, num_adversaries)
    }
}
