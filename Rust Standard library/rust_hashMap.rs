/* Hash map: */
// this needs to be imported from the rust collections library::
use std::collections::HashMap;

fn main(){
    println!("------------------------ Hash maps ----------------------------");

    // creating a hashmap:

    let mut map : HashMap<&str, i32> = HashMap::new();
    println!("{:?}", map);

    // note that insert can also be used to change values:
    map.insert("Lenny", 56);
    map.insert("Godwin", 45);
    map.insert("Godwin", 60);
    println!("{:?}", map);


    /*  retrieving values, we use the & because get returns us a reference of the value and not the actual value:
        Note that the value returned is an option enum, returning Some value and None if there is no match
    
    */
    let value = map.get(&"Lenny");

    println!("{:?}", value);

    // removing elements:

    map.remove(&"Lenny");
    println!("After removing \"Lenny \" the map is ->{:?}", map);

    for key in map.values(){
        println!("{:?}", key);
    }

}