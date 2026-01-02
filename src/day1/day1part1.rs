use crate::util::read_lines;
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

pub fn d1p1() {
    let commands = read_lines("day1input.txt").unwrap();
    let mut dial = 50; // initial position of 50
    let mut password_counter = 0; // number of times dial is at 0

    for command in commands {
        // "Command" is now io::Result<String>, not String
        // we need to handle the result
        match command {
            Ok(cmd) => { 
                let instruction = parse_direction(&cmd);
                dial = dial + instruction.unwrap();
                dial = dial % 100; //clamp to 0-99
                if dial == 0 {
                    password_counter += 1;
                }
            }
            Err(_) => {
                println!("Error reading line");
            }
        }
    }
    println!("password: {}", password_counter);
}
