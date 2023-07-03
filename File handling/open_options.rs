
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

fn main() {
    // Open a file with append option
    let mut data_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("data.txt")
        .expect("cannot open file");

    // Write to a file
    data_file
        .write_all("I am learning Rust!".as_bytes())
        .expect("write failed");

    println!("Appended content to a file");
    let mut file_content = String::new();

    // copy contents of the file to the empty string.
    data_file.read_to_string(&mut file_content).unwrap();
    println!("File content -> {:?}", file_content);
}