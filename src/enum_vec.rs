enum SpreadsheetCell {
    Count(i32),
    Number(f64),
    Text(String),
}

pub fn init() {
    let mut cells: Vec<SpreadsheetCell> = Vec::new();
    cells.push(SpreadsheetCell::Count(3));
    cells.push(SpreadsheetCell::Number(7.32));
    cells.push(SpreadsheetCell::Text(String::from("This is a comment")));
    for c in &cells {
        print(c);
    }
}

fn print(cell: &SpreadsheetCell) {
    match cell {
        SpreadsheetCell::Count(count) => println!("Count is {count}"),
        SpreadsheetCell::Number(number) => println!("Number is {number}"),
        SpreadsheetCell::Text(text) => println!("Text is {text}"),
    }
}
