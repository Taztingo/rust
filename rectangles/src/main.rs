#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32
}

fn main() {
    let rect = Rectangle { length: 50, width: 30 };

    println!(
        "The area of a rectangle is {} square pixels.",
        area(&rect)
    );
    println!("Attempting to print the rectangle {:#?}" , rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}
