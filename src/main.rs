mod day1;
mod util;
fn main() {
    if let Err(e) = day1::day1part1::d1p1() {
        eprintln!("Error: {}", e);
    }

}
