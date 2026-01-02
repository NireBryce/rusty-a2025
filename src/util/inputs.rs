use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a file and returns an iterator containing it's lines.
/// Each line wraps with Result to handle I/O errors.
// pub fn read_lines<P>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>> 
// where P: AsRef<Path>, // allows us to accept any type that implements AsRef<Path>, 
                      // look this up later because I don't know it yet
pub fn read_lines(source_path: &str) -> io::Result<Vec<String>> {
    let path = Path::new(source_path);
    let file = File::open(path?);

    // Wrap the file in a BufReader for faster line-by-line reading
    //    without buffering, each read syscall would be expensive.
    let reader = io::BufReader::new(file);

    // return an iterator where each item is Result<String>
    reader.lines().collect()
}


