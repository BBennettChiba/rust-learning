use unicode_segmentation::UnicodeSegmentation;
mod hash_maps;
fn main() {
    let a = [1,2,3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v2 = vec![1,2,3];

    let mut third = &v2[2];
    // v2.push(4);
    // println!("{}", third);

    match v.get(20){
        Some(third) => println!("{}", third),
        None => println!("None"),
    }

    for i in &v {
        // println!("{}", i);
    }

    for i in &mut v {
        *i += 1;
    }

    // println!("{:?}", v);

    enum SpreadsheetCell{
        Int (i32),
        Float (f64),
        Text (String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text("hello".to_string()),
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(i) => println!("int: {}", i),
            SpreadsheetCell::Float(f) => println!("float: {}", f),
            SpreadsheetCell::Text(s) => println!("string: {}", s),
        }
    }

    let mut str = String::from("foo");
    str.push_str("bar");
    str.push('!');
    println!("{}", str);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s4 = format!("{}{}", s1, s2);
    let s3 = s1 + &s2;
    println!("{}", s3);
    println!("{}", s4);


    let japanese = String::from("今日は！");
    for i in japanese.bytes(){
        println!("{}", i);
    }

    for i in japanese.chars(){
        println!("{}", i);
    }

    for i in japanese.graphemes(true){
        println!("{}", i);
    }

    hash_maps::run();
}


