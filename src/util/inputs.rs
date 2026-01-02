use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

/// Reads a file and returns an iterator containing it's lines.
/// Each line wraps with Result to handle I/O errors.
pub fn read_lines<P>(path: P) -> io::Result<impl Iterator<Item = io::Result<String>>> 
where P: AsRef<Path>, // allows us to accept any type that implements AsRef<Path>, 
                      // look this up later because I don't know it yet
{
    let file = File::open(path?);

    // Wrap the file in a BufReader for faster line-by-line reading
    //    without buffering, each read syscall would be expensive.
    let reader = BufReader::new(file);

    // return an iterator where each item is Result<String>
    Ok(reader.lines())
}


