#[derive(Debug)]
enum Colour{
    RED, 
    GREEN, 
    BLUE    
}

fn main(){
    /*  
        Pattern matching is a way to match the structure of a value and bind variables to its parts.
        match VALUE {
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,
            PATTERN => EXPRESSION,}
        }
        This is most commonly used to unwrap Result and Option enum type variants.
    
     */

    let y = 2;
    // if the match arms do not cater for all the cases , the firmware will not compile
     let result = match y{
        1 =>println!("You have selected one!"),
        2 =>println!("You have selected two!"),
        3 =>println!("You have selected three!"),
        _ =>println!("You have selected something else!"),
    };

    // println!("{:?}", result);
    let my_colour = Colour::GREEN;

    
    match my_colour{
        Colour::GREEN => println!("The colour is {:?}", my_colour),
        _=>println!("The colour is sth else")
    }

    // if let expressions:
    /*
        if let is a shorthand in rust for a match expression with only one pattern/ arm to match..
     */

    if let my_colour = Colour::GREEN{
        println!("The colour is green");
    } else{
        println!("The colour is not green!");
    }

    let my_option: Option<i32> = Some(222);

    // use of if let expression on the Option type
    if let Some(value) = my_option {
        println!("The option has a value of {}", value);
    } else {
        println!("The option has no value");
    }

    let number = 4;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11| 13 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        
        _ => println!("Ain't special"),
        // TODO ^ Try commenting out this catch-all arm
    }

    // note that match is an expression too:
    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };
    println!("{} -> {}", boolean, binary);


}