use std::num::ParseIntError;

fn parse_int(x : &str, y: &str)-> Result< i32, ParseIntError>{
    let int_x: i32 = x.parse()?;

    let int_y : i32 = y.parse()?;
    Ok(int_x + int_y)
}

fn main(){
    /*
        The ? operator is shorthand for returning a Result type:
        Its is only defined for Result and Opton types
    
     */
    let res = parse_int("1", "2r");
    println!("{:?}", res);
}