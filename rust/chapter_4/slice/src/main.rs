fn main() {
    let mut hello_world = String::from("Hello, world!");
    let first_word_of_hello_world = first_word(&hello_world);
    let first_word_of_world = first_word(&hello_world[7..]);

    // hello_world.clear();

    println!("{}", first_word_of_hello_world);
    println!("{}", first_word_of_world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
