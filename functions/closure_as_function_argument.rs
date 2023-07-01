fn main(){
    // closure to square a variable:

    let square = |x : i32|{ x * x};

    //call add_one with closure as argument:
    let result = add_one(20, square);
    println!("{result}");
}

fn add_one<T: Fn(i32)-> i32>( x : i32 , f: T)->i32{
    return f(x) + 1;
}