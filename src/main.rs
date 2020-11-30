use std::env;

use ben_or_randomized_consensus::{simulate, Behavior, ByteChannel, MessageChannel, Transport};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let bin_name = args.remove(0);
    match parse(args) {
        Ok((num_processes, num_zeros, num_adversaries, behavior, transport_type)) => {
            for (id, outcome) in simulate(
                num_zeros,
                num_adversaries,
                behavior,
                transport(&transport_type, num_processes),
            ) {
                println!("Process {}: outcome: {}", id, outcome);
            }
        }
        Err(e) => {
            eprintln!("Error parsing args: {}", e);
            eprintln!(
                "Usage: {} <number of nodes> <number of zeros> <number of adversaries> <behavior> <transport type>",
                bin_name
            );
            eprintln!(
                "behavior: correct|crashes|sends_invalid_messages|stops_executing|randomly_adversarial",
            );
            eprintln!("transport type: message_channel|byte_channel",);
        }
    }
}

fn parse(args: Vec<String>) -> Result<(usize, usize, usize, Behavior, String), String> {
    if args.len() != 5 {
        return Err(String::from("need exactly 5 args"));
    }

    let parse_usize = |s: &str| s.parse().map_err(|e| format!("{}", e));
    Ok((
        parse_usize(&args[0])?,
        parse_usize(&args[1])?,
        parse_usize(&args[2])?,
        args[3].parse()?,
        args[4].clone(),
    ))
}

fn transport(transport_type: &str, num_processes: usize) -> Vec<Box<dyn Transport>> {
    match transport_type {
        "message_channel" => MessageChannel::new(num_processes),
        "byte_channel" => ByteChannel::new(num_processes),
        _ => panic!("invalid transport type string"),
    }
}
