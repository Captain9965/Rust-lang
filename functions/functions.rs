fn main(){
    let number1 = 6;
    let number2 = 10;
   println!("{}", num_sum(number1, number2));
   println!("{}", return_expression(number1, number2));
   println!("{:?}", return_multiple_vals(number1, number2));
   println!("--------------- Pass by reference in Rust: ----------------------------");
   let name = String::from("Lenny");
   println!("{} is of length {}", name, calculate_length(&name));
}

fn num_sum(num1: i16,num2: i16)->i16{
    return num1 + num2;
}

fn return_expression(num1: i16,num2: i16)->i16{
    num1 + num2
}

fn return_multiple_vals(num1: i16, num2: i16)-> (i16, i16){
    (num1 + num2, num2 - num1)
}

fn calculate_length(s: &String)-> usize{
    return s.len()
}


