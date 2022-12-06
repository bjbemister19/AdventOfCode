use std::collections::VecDeque;
use std::fs;

fn contains_duplicates(header: &VecDeque<char>) -> bool {
    for (i, c) in header.iter().enumerate() {
        for h in header.iter().skip(i + 1) {
            if c == h {
                return true;
            }
        }
    }
    false
}

fn prefill_pattern_buffer(
    input_message: &str,
    pattern_buffer: &mut VecDeque<char>,
    pattern_size: usize,
) {
    // Dummy value, gets popped at the beginning of main loop, did this to avoid extra if statement
    pattern_buffer.push_back('a');
    for (i, c) in input_message.chars().enumerate() {
        pattern_buffer.push_back(c);
        if i == pattern_size - 2 {
            break;
        }
    }
}

fn detect_non_repeating_pattern(input_message: &str, pattern_size: usize) -> Option<usize> {
    if pattern_size <= 3 || input_message.len() <= pattern_size {
        return None;
    }

    let mut pattern_buffer: VecDeque<char> = VecDeque::with_capacity(pattern_size);
    prefill_pattern_buffer(input_message, &mut pattern_buffer, pattern_size);

    for (i, c) in input_message.chars().skip(pattern_size - 1).enumerate() {
        pattern_buffer.pop_front();
        pattern_buffer.push_back(c);
        if !contains_duplicates(&pattern_buffer) {
            println!("{:?}", pattern_buffer);
            return Some(i + pattern_size);
        }
    }
    None
}

fn detect_start_of_packet(input_message: &str) -> Option<usize> {
    return detect_non_repeating_pattern(input_message, 4);
}

fn detect_start_of_message(input_message: &str) -> Option<usize> {
    return detect_non_repeating_pattern(input_message, 14);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_message = fs::read_to_string("data/input.txt")?;

    let start_of_packet =
        detect_start_of_packet(&input_message).ok_or("could not find head of packet")?;
    println!("Packet detected at {}", start_of_packet);

    let start_of_message =
        detect_start_of_message(&input_message).ok_or("could not find start of message")?;
    println!("Packet detected at {}", start_of_message);

    Ok(())
}
