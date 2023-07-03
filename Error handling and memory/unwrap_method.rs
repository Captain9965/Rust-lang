fn main(){
    /*
        unwrap method():
        It returns the result of an operations for both the Option and Result types.
        It will panic when it encounters a None or an Err type

     */
    // use of a match expression:
    // let user_option = get_user("");
    
    // let result = match user_option{
    //     Some(user)=> user,
    //     None =>"not found"
    // };

    // use of unwrap method:
    // let result = get_user("Lenny").unwrap();

    // we can use expect to return a panic with a custom message:

    let result = get_user("").expect("Find user");

    println!("Username is {}", result);
}

fn get_user(username : &str)->Option<&str>{
    if username.is_empty(){
        return None;
    } else{
        return Some(username)
    }
}