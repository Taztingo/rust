fn main() {
    println!("Hello, world!");
    another_function(5, 6);
    println!("The function value is {}", five());
    println!("Plus one value is {}", plus_one(1));
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}