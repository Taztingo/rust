fn main() {
    //S1 is has to be mutable
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    //Function call needs a mutable reference
    change(&mut s1);
    println!("The string is now '{}'", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//parameter should be marked as mutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}