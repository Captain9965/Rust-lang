fn main(){
    /*
        References allow us to point to a resource without owning it. 
        Creating a reference without taking ownership of it is called borrowing.
     */

    let mut name = String::from("Lenny");

    let size = calculate_new_size(& mut name);

    println!("Size of {} is {}", name, size);

    /*
        Some rules:
            1. if we have a mutable reference to a value, we can have no other references to the same value;
    
     */
}

fn calculate_new_size(string : & mut String)->usize{
    // here, string, is not dropped, because it has no ownership of the resource.
    string.push_str(" Weda");
    string.len()
}