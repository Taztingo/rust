fn main() {
    //let number_list = vec![34, 50, 25, 100, 65];
    //let result = largest(&number_list);
    //println!("The largest number is {}", result);

    //let char_list = vec!['y', 'm', 'a', 'q'];
    //let result = largest(&char_list);
    //println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let both = Point { x: 1.0, y: 5 };
}

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point<V,W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y
        }
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}