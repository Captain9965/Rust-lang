#![allow(unreachable_code)]
fn main(){
    /* infinite loop:  */
    let mut y = 0;
    let result = loop{
        y += 1;
        println!("looping!!");
        if y > 10{
            break y;
        }
    };

    // returning a value from a loop: 
    println!("The result returned from the loop is -> {}", result);

    /* while loop:  */
    y = 0;
    while y < 5{
        y += 1;
        println!("{y}");
    }

    /* for loop 
        for exclusive, use 1..6
    */

    for i in 1..=5{
        println!(" Number is -> {i}");
    }

    //breaking out of an outer loop:
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    


}