
use std::fs::File;
use std::io::Read;
use std::io::Write;

/*

    All methods in the file struct return a variant of the Result enum..
 */
fn main(){
    // Open a file in read only mode in the local file system
    // let mut file = File::create("data.txt");
    let data_result = File::create("data.txt");

    // Reading a file returns a Result enum
    // Result can be a file or an error
    let mut data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file: {:?}", data_file);

    let mut file_content = String::new();

    // copy contents of the file to the empty string.
    // data_file.read_to_string(&mut file_content).unwrap();
    // println!("File content -> {:?}", file_content);

    data_file.write("Yes".as_bytes()).expect("write failed");

    // remove file:
    // fs::remove_file("data.txt").expect("could not remove file");
}