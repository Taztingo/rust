fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}