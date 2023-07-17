/*
    Attributes are used for:
        1. conditional compilation
        2. disable warnings
        3. link to a foreign library
        4. set crate name, library, and type
        5. Mark functions as unit tests
        6. attributes like macros

        #![crate_attribute] -> apply to a whole crate
        #[item_attribute] ->apply to a module or item.

        syntax: #[attribute = "value"]
        #[attribute(key = "value")]
        #[attribute(value)]

 */
#![allow(dead_code)]
// dead code attribute:
fn used_function() {}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint

fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
}