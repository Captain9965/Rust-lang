/*
    Modules help in splitting a program for better readability and organisation.
    This is a collection of items..functions, structs even other modules!

 */

 // nested module
pub mod player {
    pub mod sprite {
        pub fn create() {
            println!("called player::sprite::create");
        }
    }
}

// bring the create function into scope
use player::sprite::create;

fn main() {
    // call public function directly
    create();
}