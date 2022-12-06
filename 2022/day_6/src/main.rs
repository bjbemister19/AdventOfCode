use std::fs;

fn contains_duplicates(header: &str) -> bool {
    for (i, c) in header.chars().enumerate() {
        for h in header.chars().skip(i + 1) {
            if c == h {
                return true;
            }
        }
    }
    false
}

fn detect_non_repeating_pattern(input_message: &str, pattern_size: usize) -> Option<usize> {
    if input_message.len() <= pattern_size {
        return None;
    }

    for (i, c) in input_message.chars().skip(pattern_size).enumerate() {
        let pos = i+pattern_size;
        let header = &input_message[i..pos];
        if !contains_duplicates(&header) {
            println!("{:?}", header);
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
    println!("Message detected at {}", start_of_message);

    Ok(())
}
