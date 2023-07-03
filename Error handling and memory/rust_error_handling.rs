fn main(){
    // errors with panic:
    println!("------------- This is an example of panic --------------");
    // panic!("Oooh, no! :(");
    /* we also have the option enum: 
        Option <T> is an enum type with 2 possible results:
            1. None : failure with no value
            2. Some (T): A value with type T
    
    
     */

    let text = "Hello World";
    let character_option = text.chars().nth(11);

    // using match for option type:
    let character = match character_option{
        None => "empty".to_string(),
        Some(c)=> c.to_string()
    };
    println!("Character at index 11 is -> {}", character);

    println!("---------------------- Using the result enum ---------------------------");

    use std::fs::File;

    let data_result = File::open("data.txt");

    // using match for Result type
    let data_file = match data_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the data file: {:?}", error),
    };

    println!("Data file : {:?}", data_file);

}