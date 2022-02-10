struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("top: {}", top);
    }

    let point = (1, 2, 3);
    let (x, y, z) = point;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    print_coordinates(&point);

    if let Some(a) = Some(3) {
        println!("a: {}", a)
    }

    println!("matching...");
    match x {
        1 | 11 | 111 => println!("1/11/111"),
        2 => println!("2"),
        _ => println!("#"),
    }

    match x {
        1..=5 => println!("1..=5"),
        6..=10 => println!("6..=10"),
        _ => println!("other"),
    }

    let point = Point { x: 1, y: 1 };
    let Point { x: p, y: q } = point;
    println!("p,q: {},{}", p, q);

    match point {
        Point { x: 1, y } if y < 10 => println!("x,y = 1,{} (y small)", y),
        Point { x: 1, y } => println!("x,y = 1,{}", y),
        Point { x: 2, y } => println!("x,y = 2,{}", y),
        _ => println!("no matches"),
    }

    match point {
        Point { x, .. } => println!("x: {}", x),
    }

    match point {
        Point {
            x: x_value @ -100..=100,
            ..
        } => println!("matched, {}!", x_value),
        _ => println!("no match!"),
    }
}

fn print_coordinates(&(x, y, z): &(i32, i32, i32)) {
    println!("coordinates: ({}, {}, {})", x, y, z);
}
