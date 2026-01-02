use crate::util::inputs::read_lines;

fn parse_direction(command: &str) -> Option<i8> {
    if let Some(suffix) = command.strip_prefix("R") {
        let suffix: i8 = suffix.parse().unwrap();
        return Some(suffix);
    } else if let Some(suffix) = command.strip_prefix("L") {
        let suffix: i8 = suffix.parse().unwrap();
        return Some(suffix);
    }

    None // No match
}

pub fn main() {
    let commands: Vec<String> = read_lines("day1input.txt").unwrap();
    let mut dial = 50; // initial position of 50
    let mut password_counter = 0; // number of times dial is at 0

    // watch out, by-character is complicated in rust! Unicode default encoding
    for command in commands {
        let instruction = parse_direction(&command);

        dial = dial % 100; //clamp to 0-99

        if dial == 0 {
            password_counter += 1;
        }
    }
    println!("password: {}", password_counter);
}
