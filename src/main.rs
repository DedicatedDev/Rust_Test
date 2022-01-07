fn main() {
    let mut x = 5;
    println!("The value of x is {}",x);
    x = 6;
    println!("The value of x is {}",x);
    const THREE_HOURS_SECONDS: u32 = 60 * 60 * 3;
    print!("{}",THREE_HOURS_SECONDS);
    let guess: u32 = "42".parse().expect("Not a number");
    print!("{}",guess);
}
