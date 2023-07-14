fn main(){
    /* Rust structs:  */
    struct NameStruct {
        name: String,
        age: u8,
        height : u8
    }

    let person = NameStruct{
        name: String::from("Lenny"), 
        age: 20,
        height: 78
    };
    let person_two = NameStruct{
        name: String::from("Weda"),
        ..person // instantiating using the struct update syntax to make structs have similar fields    
    };

    println!("{}", person.name);
    println!("{}", person_two.age);
    /*
    destructuring fields of a struct
    The name of the variable must be the name of the struct's fields
    
    */

    let NameStruct{ name, age, height} = person;

    println!("The name is {}, while the age is {}, and the height is {}", name, age, height);

    #[derive(Debug)]
    // tuple structs:
    struct Point(i16, i16);

    let point_instance = Point(45, 67);
    println!("{:?}", point_instance);
}