use std::io;

fn main() {
    println!("Enter a list of numbers separated by a space");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
            .expect("Failed to read line");
    let mut list: Vec<i32> = Vec::new();
    for number in input.split_whitespace() {
        list.push(number.parse::<i32>().unwrap());
    }
    println!("The mean is: {}", calculate_mean(&list));
    println!("The median is: {}", calculate_median(&list));
    println!("The mode is: {}", calculate_mode(&list));
    println!("The pig latin of {} is {}", "first", pig_latin("first"));
}

fn calculate_mean(list: &Vec<i32>) -> f32 {
    let mut sum = 0;
    for x in list.iter() {
        sum += *x;
    }
   (sum as f32) / (list.len() as f32)
}

fn calculate_median(list: &Vec<i32>) -> f32 {
    let mut copy = list.clone();
    copy.sort();
    let middle = ((copy.len() / 2) as usize);
    let mut median = 0.0;
    if copy.len() % 2 == 0 {
        median = ((copy[middle] + copy[middle-1]) as f32) / 2.0;
    } else {
        median = copy[middle] as f32;
    }
    median
}

fn calculate_mode(list: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut max = -1;
    let mut mode = -1;
    for x in list.iter() {
        let count = map.entry(*x).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
            mode = *x;
        }
    }
    mode
}

fn pig_latin(string: &str) -> String {
    let mut copy = String::from(string);
    let firstLetter = copy.remove(0);
    let secondLetter = copy.chars().next();
    match secondLetter {
        None => println!("Error with string!"),
        Some(x) => {
            let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
            if vowels.contains(&x) {
                copy.push('-');
                copy.push(firstLetter);
                copy.push_str("ay");
            } else {
                copy.push(firstLetter);
            };
        }
    }
    copy
}