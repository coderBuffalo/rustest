fn main() {
    let x = 5;
    let y: Option<u32> = None;

    println!("{}", x + y.unwrap_or(0));
}