fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let mut number2 = 3;
    loop {
        if number2 == 0 {
            break;
        }
        println!("{}!", number2);
        number2 -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
