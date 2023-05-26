fn main(){
    let y = 3.14;
    let x = 25;
    let z = "349";

    /* one can specify the placeholder numbering in rust:  */
    println!("The numbers are {2} ,{1} and {0}", x, y, z);

    /* passing the variables directly:  */
    println!("Rust is an extremely fast language. I learnt it in\
     {x} days ");

     /* escape sequences:  */
     println!("This is the first line \nThis is a new line");
}