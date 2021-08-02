use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub struct Result {
    pub seed: u32,
    pub sequences: u32
}

pub fn start(start: u32, end: u32) -> Receiver<Result> {
    let (tx, rx) = channel::<Result>();
    thread::spawn(move || {
        for seed in start..(end+1) {
            tx.send(calculate_sequences(seed));
        }
    });
    rx
}

fn calculate_sequences(start: u32) -> Result {
    let mut sequence: u32 = 0;
    let mut value: u32 = start;

    while value > 1 {
        if (value % 2) == 1 {
            value = (3 * value) + 1;
        } else {
            value /= 2;
        }
        sequence += 1;
    }
    return Result{ seed: start, sequences: sequence};
}

#[test]
fn test_start_10() {
    let rx = start(10, 10);
    let seq10 = rx.recv().unwrap();
    assert_eq!(6, seq10.sequences);
}

#[test]
fn test_start_10_to_11() {
    let rx = start(10, 11);
    for result in rx {
        match result.seed {
            10 => assert_eq!(6, result.sequences),
            11 => assert_eq!(14, result.sequences),
            _ => assert!(false),
        }
    }
}

#[test]
fn test_calc_seq_10() {
    let seq10 = calculate_sequences(10);
    assert_eq!(6, seq10.sequences);
}

#[test]
fn test_calc_seq_123() {
    let seq123 = calculate_sequences(123);
    assert_eq!(46, seq123.sequences);    
}