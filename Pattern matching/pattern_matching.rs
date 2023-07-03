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



}