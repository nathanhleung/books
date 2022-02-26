fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("Pushed: {}", s);

    take_ownership(s.clone());
    println!("Moved: {}", s);

    let mut s1 = String::from("Hello");
    let s2 = s1.clone();
    println!("Moved: {}, world!", s1);

    println!("Length: {}", calculate_length(&s2));

    let mutable_ref1 = &mut s1;
    let mutable_ref2 = mutable_ref1;
    change(mutable_ref2);
    change(&mut s1);
    println!("Changed: {}", s1);
}

fn take_ownership(s: String) -> String {
    s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
