fn main(){
    /* infinite loop:  */
    let mut y = 0;
    loop{
        y += 1;
        println!("looping!!");
        if y > 1000{
            break;
        }
    }

    /* while loop:  */
    y = 0;
    while y < 5{
        y += 1;
        println!("{y}");
    }

    /* for loop */

    for i in 1..6{
        println!(" Number is -> {i}");
    }

    


}