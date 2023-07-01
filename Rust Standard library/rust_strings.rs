fn main(){

    /*
        It is allocated in the heap and it is dynamic in size...which is unkown at compile time.
    */
    let mut name : String = String::from("Lenny");
    name.push_str(" 254");
    let length = name.len();

    let slice = & mut name[0..length];
    println!("{}", slice);

    // iterating over a string:

    for char in name.chars(){
        println!("{}", char);
    }

    // using dynamic memory:

    let mut new_string = String::new();
    new_string.push_str("new string");

    println!("{}", new_string);


    // Note that Rust does not support string indexing
}