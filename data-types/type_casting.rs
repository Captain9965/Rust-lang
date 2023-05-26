fn main(){

    /* type casting */
    let x : f64 = 56.43;
    let u = x as i8;
    println!("{x} has been casted explicitly to {u}");

    /* unsigned int to char :  */

    let i :u8 = 89;
    let c : char = i as char;
    println!("{i} converts to {c}");

    /* char to unsigned int */
    let u : char = 'r';
    let h : u8 = u as u8;

    println!("{u} converts to {h}");
    /* Note that typecasting has its limits. For example, we cannot convert a float to char type */

}