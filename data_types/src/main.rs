fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let y: f32 = 3.0;

    let addition = 5 + 10;
    let subtraction = 95.5 - 4.3;
    let product = 4 * 30;
    let division = 56.7 / 32.2;
    let remainder = 43 % 5;
    let boolean = true;

    let c = 'z';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("The value of y is: {}", y);
    println!("The value for index 0 is: {}", tup2.0);
    println!("The value for index 1 is: {}", tup2.1);
    println!("The value for index 2 is: {}", tup2.2);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    let error = a[100];
}
