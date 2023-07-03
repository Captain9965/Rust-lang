/*
    Rust Generics can be used to write code that is flexible, and can be reused with different data types.
    An example of this is the HashMap type, which allows us to use any data types...
    Generic types use single characters unlike actual concrete types which use String, &str, i32 etc
    As a convention, use T and U for arbitrary types, K and V for key, value pairs and E for errors.
 */

// Generic struct:
#[derive(Debug, Clone)]
struct Point <T>{
    x : T,
    y : T
}

fn main(){
    let int_point = Point{x: 2, y: 3};
    let float_point = Point{x: 5.6,y : 5.4};
    let copy_object = float_point; // demos

    println!("Generic structs with int ->{:?} and float ->{:?}", int_point, copy_object);

    println!("The minimum values are {} and {}", min(4, 5), min(9.5, 6.5))
}

// initialize with a trait which provides methods for comparing values of a type.
fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        return a;
    } else {
        return b;
    }
}
