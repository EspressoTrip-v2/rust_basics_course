use rand::Rng;

use modules::greet;

fn main() {
    let x = rand::thread_rng().gen_range(1..100);
    greet();
    println!("Random number: {}", x)
}


