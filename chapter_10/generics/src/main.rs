struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_i32 = largest_i32(&number_list);
    println!("The largest number is {}", largest_i32);

    let char_list = vec!['y', 'm', 'n', 'q'];
    let largest_char = largest_primitive(&char_list);
    println!("The largest char is {}", largest_char);

    let p1 = Point { x: 5.0, y: 10.0 };
    println!("p1.x = {}", p1.x());
    println!("Distance from origin = {}", p1.distance_from_origin());
    let p2 = Point { x: 'x', y: 'y' };
    println!("p2.x = {}", p2.x());

    let string_list = vec![
        String::from("zzz"),
        String::from("hello"),
        String::from("world"),
    ];
    let largest_string = largest(&string_list);
    println!("The largest string is {}", largest_string);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    println!(
        "The longest string is {}",
        longest(string1.as_str(), string2)
    );

    let longest_str;
    {
        let string3 = String::from("qwerty");
        longest_str = longest(string1.as_str(), string3.as_str());
        println!("{} is also a long string", longest_str);
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_primitive<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if *item > *largest {
            largest = &item;
        }
    }

    &largest
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
