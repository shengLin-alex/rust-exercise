#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    let mut v1: Vec<i32> = Vec::new();

    v1.push(1);
    v1.push(2);

    let mut v2 = vec![1,3,4,5];

    let third = v2[2];
    let third: Option<&i32> = v2.get(3);

    v2.push(7);

    let third = v2[2];

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![32,43,12,56,897];
    for i in &mut v {

        // use * to de-reference
        *i += 60;
    }

    for i in &v {
        println!("{}", i);
    }

    for e in &storage_enum() {
        println!("{:?}", e);
    }
} // 離開scope vector 與其元素將一併失效

fn storage_enum() -> Vec<SpreadsheetCell> {

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    row
}