fn main() {
    // let x = 5;
    // {
    //     let y = 99;
    //     println!("x = {}, y = {}", x, y);
    // }
    // println!("x = {}, y = {}", x, y);

    // Shadowing
    let x = 5;
    {
        let x = 99;
        println!("x = {}", x);
    }
    println!("x = {}", x, );

    let meme = "string";
    let meme = meme.len();
    println!("meme = {}", meme);

    let num: i32;
    num = 5;
    num = 6;
}
