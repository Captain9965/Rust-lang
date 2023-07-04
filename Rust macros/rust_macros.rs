/*

    A macro is a piece of code that generates another piece of code.
    It allows us to write more code that writes more code.-> meta programming
    common macros include vec!, println! and panic!
 */
// macro definition with one rule match: 
macro_rules! hello_world{
    //takes no arguments
    ()=>{
        println!("Hello World!")
    };
}

macro_rules! print_anything{

    /*
        other designators that can be used include:
            1. stmt - statement
            2. pat - pattern
            3. expr - expression
            4. ty - type
            5. ident - identifier.
    
     */
    ($message : expr)=>{
        println!("{}", $message)
    };
}

// a macro that uses repetition:
macro_rules! repeat_print{
    // match rule which matches multiple expressions in an argument:
    /*
        ($($x : expr), *) is a repeating pattern consisting of zero or more expressions, separated by commas
        that are matched by the macro.
        the * at the end will repeatedly match against the pattern inside $()
        the function inside $()* will repeated zero or more times, once for each expression in the list of arguments.
    
     */
    ($($x : expr), *) => {
        $(
        println!("{}", $x);
    )*
    };
}


fn main(){
    hello_world!();
    print_anything!("good");
    repeat_print!(1, 2, 3);
}