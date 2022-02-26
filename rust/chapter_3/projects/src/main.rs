use std::io;

fn main() {
    println!("Please input degrees Fahrenheit.");
    let mut fahrenheit = String::new();
    io::stdin()
        .read_line(&mut fahrenheit)
        .expect("Failed to read line");

    let fahrenheit = fahrenheit.trim().parse::<f64>().expect("Failed to parse");
    println!("Degrees Celsius: {}", fahrenheit_to_celsius(fahrenheit));

    println!("Please input the index of the Fibonacci number.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index = index.trim().parse::<u32>().expect("Failed to parse");
    println!("Fibonacci number #{}: {}", index, fibonacci(index));

    twelve_days_of_christmas();
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fibonacci(n: u32) -> u64 {
    if n <= 2 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "And a partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    for (i, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me", day);

        if i == 0 {
            println!("A partridge in a pear tree.");
        } else {
            for j in (0..(i + 1)).rev() {
                println!("{}", gifts[j]);
            }
        }
        println!("");
    }
}
