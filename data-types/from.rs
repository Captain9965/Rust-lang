use std::convert::From;


#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let val = 5;
    let num0: Number = val.into();
    println!("My number is {:?}", num0);
    let num = Number::from(30);
    println!("My number is {:?}", num);
}