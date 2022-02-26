fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    let rect2 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect2)
    );

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect3)
    );

    println!("The rectangle is {:#?}", rect3);

    let scale = 2;
    let rect4 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect4);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect4.area()
    );

    let rect5 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect6 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect7 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect5 hold rect6? {}", rect5.can_hold(&rect6));
    println!("Can rect5 hold rect7? {}", rect5.can_hold(&rect7));

    let square1 = Rectangle::square(10);
    println!("The are of the square is {} square pixels.", square1.area())
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
