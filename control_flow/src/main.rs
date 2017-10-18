fn main() {
    let number = 7;

    if number < 5 {
        println!("Less than 5");
    } else if number == 5 {
        println!("Equal to 5");
    } else {
        println!("Greater than 5");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number: {}", number);
}
