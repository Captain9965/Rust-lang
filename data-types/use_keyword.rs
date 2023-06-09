// An attribute to hide warnings for unused code.
#![allow(dead_code)]
#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

#[derive(Debug)]
enum Work {
    Civilian,
    Soldier,
}
// note that the structs above use implicit discriminators ( begin at zero )
fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }

    println!("{}", Rich as i32);
}