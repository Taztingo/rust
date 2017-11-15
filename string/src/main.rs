fn main() {
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut lol = String::from("lo");
    lol.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;

    let s4 = String::from("Hellos");
    for c in s4.chars() {
        println!("{}", c);
    }
}
