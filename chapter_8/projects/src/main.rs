use std::collections::HashMap;
use std::io;

fn main() {
    let ints = vec![1, 2, 3, 3];
    println!("{:?}", measures_of_central_tendency(&ints));

    println!("Please enter a sentence to translate to pig latin.");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Translated: {}", sentence_to_pig_latin(&input.trim()));
    add_employees();
}

// Returns a tuple of (mean, median, mode)
fn measures_of_central_tendency(ints: &Vec<i32>) -> (f64, f64, i32) {
    let len = ints.len();
    if len == 0 {
        return (0.0, 0.0, 0);
    }

    let mut sum = 0;
    let mut counts = HashMap::new();

    for &i in ints {
        sum += i;
        let entry = counts.entry(i).or_insert(0);
        *entry += 1;
    }

    // TODO(nathanhleung): handle multiple modes
    let mut max_count = 0;
    let mut mode = 0;
    for (key, count) in counts {
        if count > max_count {
            mode = key;
            max_count = count;
        }
    }

    let mean = sum as f64 / len as f64;

    let mut sorted = ints.to_vec();
    sorted.sort();
    let median = if len % 2 == 0 {
        (sorted[len / 2] + sorted[len / 2 + 1]) as f64 / 2.0
    } else {
        sorted[len / 2] as f64
    };

    (mean, median, mode)
}

fn sentence_to_pig_latin(string: &str) -> String {
    string
        .split(' ')
        .map(|word| word_to_pig_latin(word) + " ")
        .collect()
}

fn word_to_pig_latin(string: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let string = string.to_lowercase();
    let mut chars = string.chars();
    let some_first_char = chars.next();
    let rest: String = chars.collect();

    let first_char_is_vowel = if let Some(first_char) = some_first_char {
        vowels.contains(&first_char)
    } else {
        false
    };

    let first_char_str = if let Some(first_char) = some_first_char {
        first_char.to_string()
    } else {
        String::from("")
    };

    if first_char_is_vowel {
        format!("{}hay", string)
    } else {
        format!("{}{}ay", rest, first_char_str)
    }
}

fn add_employees() {
    let mut employees_by_department: HashMap<String, Vec<String>> = HashMap::new();

    println!("Please input the employee and their division.");
    println!("e.g. \"Add [name] to [department]\"");
    println!("Type \"break\" to finish.");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "break" {
            break;
        }

        let words: Vec<&str> = input.split(' ').collect();
        if words.len() != 4 {
            println!("Invalid input!");
            continue;
        }

        let name = words[1].trim();
        let department = words[3].trim();

        employees_by_department
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(name.to_string());

        println!("Added {} to {}!", name, department);
    }

    let mut departments: Vec<String> = employees_by_department.keys().cloned().collect();
    departments.sort();

    for department in departments {
        println!("Employees in {}:", department);

        if let Some(employees) = employees_by_department.get_mut(&department) {
            employees.sort();
            println!("{:?}", employees);
        } else {
            println!("No employees");
        }
    }
}
