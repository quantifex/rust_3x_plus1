use crate::collatz::*;
use std::{cmp, env};

mod collatz;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Collatz Brute Force Calculator (v0.1)");
    if args.len() != 3 {
        println!("Required input arguments missing.\nPlease provide 2 seed values (start and end)");
        println!("\t{} start end", args[0]);
        return;
    }

    let first: u32 = args[1].parse().unwrap();
    let second: u32 = args[2].parse().unwrap();
    let start_seed = cmp::min(first, second);
    let end_seed = cmp::max(first, second);

    let rx = start(start_seed, end_seed);
    for result in rx {
        println!("Seed: {} \tSequences: {}", result.seed, result.sequences);
    }
}
