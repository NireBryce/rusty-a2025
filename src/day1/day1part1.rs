use crate::util::inputs::read_lines;
use std::io;

fn parse_direction(command: &str) -> Result<i16, String> {
    if let Some(suffix) = command.strip_prefix("R") {
        let suffix: i16 = suffix.parse()
            .map_err(|_| format!("Invalid number after R: {}", suffix))?;
        return Ok(suffix);
    } else if let Some(suffix) = command.strip_prefix("L") {
        let suffix: i16 = suffix.parse()
            .map_err(|_| format!("Invalid number after L: {}", suffix))?;
        return Ok(-suffix);
    }
    
    Err(format!("Invalid command: {}", command))
}

pub fn d1p1() -> io::Result<()> {
    let commands = read_lines("day1/day1input.txt")?;
    let mut dial: i16 = 50;
    let mut password_counter = 0;

    for command in commands {
        let cmd = command?;
        match parse_direction(&cmd) {
            Ok(instruction) => {
                dial += instruction;
                dial = dial.rem_euclid(100);
                println!("dial: {}", &dial);
                if dial == 0 {
                    password_counter += 1;
                }
            }
            Err(e) => eprintln!("Warning: {}", e),
        }
    }
    println!("password: {}", password_counter);
    
    Ok(())
}
