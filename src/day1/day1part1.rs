use std::io::{self, BufRead};
use std::fs::File;

mod util; 
use super::util::read_lines::read_lines;

fn parse_direction(command: &str) -> Option<(char, &str)> {
    if let Some(suffix) = command.strip_prefix("R") {
        let suffix: i8 = suffix.parse().unwrap();
        return Some((suffix));
    } 
    
    else 
    if let Some(suffix) = command.strip_prefix("L") {
        let suffix: i8 = suffix.parse().unwrap();
        return Some(suffix) 
    }

    None // No match
}

fn d1p1() {
    const INITIAL_POSITION: u8 = 50;
    let commands = read_lines("./day1input.txt").unwrap();
    let mut dial = INITIAL_POSITION; // initial position of 50
    let mut counter = 0; // number of times dial is at 0
    
    // watch out, by-character is complicated in rust! Unicode default encoding
    for command in commands {
        let instruction = parse_direction(command.as_chars());

        dial = (dial + instruction.unwrap()).abs(); // if it goes above or below 0

        if dial == 0 {
            counter += 1;
        }
        
    }

}
