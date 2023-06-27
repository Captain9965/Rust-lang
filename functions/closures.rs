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
    let print_word = | s : String |{ println!("{}", s + " " +  &second_name);};

    print_word(first_name);


    println!("--------------------------------------- Capture by mutable borrow ---------------------");

    let mut add_word = | s : String | {
        first_name.push_str(" Weda");
        println!("{}", s);
    };

    add_word(first_name);
    println!("Outside the closure , the string is -> {}", first_name);

}