fn main(){
    /* 
        Ownership is a set of rules that ensures memory safety in rust programs:
        1. Each value in Rust has an owner.
        2. There can only be one owner at a time.
        3. When an owner goes out of scope, the value will be dropped
     */

    // sometimes we may want to transfer ownership of the variable from one binding to another and not let it be dropped:

    let fruit = String::from("Fruit"); // N/B: Note that a string stores data both on the stack and the heap:

    // move ownership:
    let fruit2 = fruit;

    // ownership has moved, cannot println fruit varlable now. In essence, 2 variables cannot point to the same content
    // if the data type does not have a fixed size in memory and uses the heap to store content.:
    println!("Fruit -> {}", fruit2);


    //ownership rules do not apply to primitives like integers, floats or boolean values. For this, since the size is known at 
    // compile time, making copies of them is fast and easy.
    let mut x = 11;
    let y = x;
    x += 1;
    println!("numbers are {} and {}", x, y);

    /*
        Ownership in functions:

    
     */

    let name = String::from("Goose");
    // ownership of name moves into the function 
    print_string(name);

    // no longer valid:
    // println!("{}", name);
    
}


fn print_string(string :String){
    println!("{}", string);
    // string goes out of scope and memory is freed..
}