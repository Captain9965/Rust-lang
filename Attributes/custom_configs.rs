/* to see this in action:
    rustc --cfg some_condition custom_configs.rs

 */

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}