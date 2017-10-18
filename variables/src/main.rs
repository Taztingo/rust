fn main() {
    let mut x = 5;
    const MAX_POINTS: u32 = 100_000;
    println!("The constant value is: {}", MAX_POINTS);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x1 = 5;
    let x1 = x1 + 1;
    let x1 = x1 * 2;
    println!("The value of x1 is: {}", x1);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("The number of spaces is {}", spaces);
}
