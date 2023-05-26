fn main(){
    let mut y = 60;

    /* in rust, constants are capitalized  */
    const U:&str = "name";
    // let y = 20;
    y = 78; /*will not work as let declares an immutable variable*/
    println!("The number is {y} and the constant is {U}");


}