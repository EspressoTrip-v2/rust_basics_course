    const STARTING_MISSILES: i32 = 8;
    const READY: i32 = 2;
fn main() {
    // let mut missiles: i32 = STARTING_MISSILES;
    // let ready: i32 = READY;
    let (ready, mut missiles): (i32, i32) = (READY, STARTING_MISSILES);
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
}
