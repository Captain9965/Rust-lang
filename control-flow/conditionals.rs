fn main(){
    let x = 78;
    let y = 67;

    /* notice the abscence of parantheses:  */
    if x < y{
        println!("x is less than y");
    } else if x > y && x < 100{
        println!("x is within limits");
    }
}