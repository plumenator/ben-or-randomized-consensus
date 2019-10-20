use std::env;

use ben_or_randomized_consensus::{simulate, Behavior};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let bin_name = args.remove(0);
    match parse(args) {
        Ok((num_processes, num_zeros, num_adversaries, behavior)) => {
            for (id, outcome) in simulate(num_processes, num_zeros, num_adversaries, behavior) {
                println!("Process {}: outcome: {}", id, outcome);
            }
        }
        Err(e) => {
            eprintln!("Error parsing args: {}", e);
            eprintln!(
                "Usage: {} <number of nodes> <number of zeros> <number of adversaries> <behavior>",
                bin_name
            );
            eprintln!(
                "behavior: correct|crashes|sends_invalid_messages|stops_executing|randomly_adversial",
            );
        }
    }
}

fn parse(args: Vec<String>) -> Result<(usize, usize, usize, Behavior), String> {
    if args.len() != 4 {
        return Err(String::from("need 4 args"));
    }

    let parse_usize = |s: &str| s.parse().map_err(|e| format!("{}", e));
    Ok((
        parse_usize(&args[0])?,
        parse_usize(&args[1])?,
        parse_usize(&args[2])?,
        args[3].parse()?,
    ))
}
