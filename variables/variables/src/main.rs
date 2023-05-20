// Error
// let num = 2;
// Pass
const NUM: i32 = 3;

fn main() {
    // Fail
    // let bunnies = 2;
    // bunnies = 3;

    // Pass
    let mut  bunnies = 2;
    bunnies = 3;
    println!("bunnies: {}", bunnies);

    // Correct syntax
    const WARP_FACTOR: f64 = 9.9;
    println!("WARP_FACTOR: {}", WARP_FACTOR);
}
