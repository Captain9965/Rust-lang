
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    // simulate rolling a die
    println!("roll = {}", rng.gen_range(1..=6));
}
