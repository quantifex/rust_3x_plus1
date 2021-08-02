fn main() {
    println!("Hello, world!");
}

fn calculate_sequences(start: u32) -> u32 {
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
    return sequence;
}

#[test]
fn test_collatz_10() {
    let seq10 = calculate_sequences(10);
    assert_eq!(6, seq10);    
}

#[test]
fn test_collatz_123() {
    let seq123 = calculate_sequences(123);
    assert_eq!(46, seq123);    
}