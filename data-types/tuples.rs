fn main(){


    /* In Rust, note that tuples are fixed in size and cannot grow or shrink after they have been created.
        The 
    
     */

    let mut tuple: (&str, u8, f32) = ("Lenny", 26, 5.6);
    println!("My name is {}, and I am {} years old. My height is {} feet", tuple.0, tuple.1, tuple.2);
    /* tuples are immutable. However, when declared as mutable, their values can be changed, albeit with the same data type */
    tuple.2 = 6.0;

    /* To print an entire tuple: */
    println!("{:?}", tuple);
    /* destructuring or unpacking a tuple: */

    let (x, y, z) = tuple;
    println!("After destructuring -> {}, {}, {}", x, y, z);


   
}