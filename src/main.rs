use std::env;

use ben_or_randomized_consensus::simulate;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let bin_name = args.remove(0);
    match parse(args) {
        Ok((num_processes, num_adversaries, num_zeros)) => {
            for (id, outcome) in simulate(num_processes, num_adversaries, num_zeros) {
                println!("Process {}: outcome: {}", id, outcome);
            }
        }
        Err(e) => {
            eprintln!("Error parsing args: {}", e);
            eprintln!(
                "Usage: {} <number of nodes> <number of adversaries> <number of zeros>",
                bin_name
            );
        }
    }
}

fn parse(args: Vec<String>) -> Result<(usize, usize, usize), String> {
    if args.len() != 3 {
        return Err(String::from("need 3 args"));
    }

    let parse_usize = |s: &str| s.parse().map_err(|e| format!("{}", e));
    Ok((
        parse_usize(&args[0])?,
        parse_usize(&args[1])?,
        parse_usize(&args[2])?,
    ))
}
