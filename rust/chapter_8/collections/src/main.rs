use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //
    // Vectors
    //
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let mut v2 = vec![1, 2, 3];

    let third = &v2[2];
    println!("The third element of v2 is: {}", third);

    v2.push(10);

    println!("The elements of v2 are:");
    for i in &mut v2 {
        println!("{}", i);
        *i += 50;
    }

    match v2.get(2) {
        Some(third) => println!("The third element of v2 is: {}", third),
        None => println!("There is no third element"),
    }

    let _spreadsheet_row = vec![
        SpreadsheetCell::Int(32),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text(String::from("#REF!")),
    ];

    let _does_not_exist = v2.get(100);
    // let _does_not_exist = &v2[100];

    //
    // Strings
    //
    let mut s1 = String::new();
    let s2 = "&str";
    let s3 = s2.to_string();
    let s4 = String::from("String");
    s1.push_str("more string");
    s1.push('!');
    let s5 = String::new() + &s1 + &s2;
    let s6 = format!("{}-{}-{}", s1, s2, s3);

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
    println!("s4: {}", s4);
    println!("s5: {}", s5);
    println!("s6: {}", s6);

    println!("Bytes");
    for b in s1.bytes() {
        print!("{} ", b);
    }
    println!();

    println!("Chars");
    for c in s1.chars() {
        print!("{} ", c);
    }
    println!();

    //
    // Hash Maps
    //
    let mut scores1 = HashMap::new();
    scores1.insert(format!("Blue"), 10);
    scores1.insert(format!("Yellow"), 50);
    println!("{:?}", scores1);

    let teams = vec![format!("Blue"), format!("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}", scores2);

    let blue = format!("Blue");
    let blue_score = scores1.get(&blue);
    if let Some(score) = blue_score {
        println!("Blue score: {}", score);
    }

    println!("Insert + for loop");
    scores1.insert(blue.clone(), 100);
    let entry = scores1.entry(blue.clone()).or_insert(150);
    *entry += 1;
    for (key, value) in scores1 {
        println!("{}: {}", key, value);
    }
}
