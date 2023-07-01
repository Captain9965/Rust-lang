fn main(){
    /* 
        These are functions without names
        Also called lambdas or anonymous functions
     */
    let print_name = || println!("Bond...James Bond");
    print_name();

    /* closures with environment capturing, parameters, supports multiline statements: */
    let mut first_name : String = String::from("Lenny");
    let second_name : String = String::from("Weda");
    println!("---------------------------- Capture by immutable borrow -----------------------------");

    let print_word = | s : String |{ println!("{}", s + " " +  &second_name);};

    print_word(first_name.clone()); // have to clone since the variable first_name is borrowed in a closure..


    println!("--------------------------------------- Capture by mutable borrow ---------------------");

    let mut add_word = | s : String | {
        first_name.push_str(&s);
        println!("{}", first_name);
    };

    add_word(second_name);
    println!("Outside the closure , the string is -> {}", first_name);

    println!("------------------------------------- Capture by move --------------------------------------");

    let vehicle = String::from("Toyota");

    //closure:
    let displayVehicle = || {
        let vehicleName = vehicle;
        println!("The vehicle name is {vehicleName}");
    };
    displayVehicle();

    // Note that this is not allowed since the value has been moved by the closure...
    // println!("Captured by move -> {vehicle}");
}