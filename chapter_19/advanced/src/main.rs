use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use std::fmt;
use std::slice;

static GLOBAL: &str = "GLOBAL";
static mut COUNTER: i32 = 0;

extern "C" {
    fn abs(input: i32) -> i32;
    fn pow(base: f64, exponent: f64) -> f64;
}

#[derive(HelloMacro)]
struct Human;

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);

    fn power();
}

impl Pilot for Human {
    fn fly(&self) {
        println!("captain");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("gandalf");
    }

    fn power() {
        println!("9000");
    }
}

impl Human {
    fn fly(&self) {
        println!("human");
    }

    fn power() {
        println!("1");
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for String {}

struct DisplayableVec(Vec<String>);

impl fmt::Display for DisplayableVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    dangerous(r1, r2);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        unsafe_dangerous(r1, r2);
    }

    let address = 0x01234usize;
    let r = address as *const i32;
    // causes segfault
    // unsafe {
    //     println!("random address contains: {}", *r);
    // }

    let mut v = vec![1, 2, 3];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(2);
    println!("a: {:?}, b: {:?}", a, b);

    let (c, d) = split_at_mut(r, 1);
    println!("c: {:?}, d: {:?}", c, d);

    unsafe {
        println!("Absolute value of -3 in C is: {}", abs(-3));
        println!("2.71828^3.14159 in C is: {}", pow(2.71828, 3.14159));
    }

    println!("{}", GLOBAL);

    unsafe {
        COUNTER += 1;
        println!("counting: {}", COUNTER);
    }

    let person = Human;
    Pilot::fly(&person);
    <Human as Pilot>::fly(&person);
    Wizard::fly(&person);
    person.fly();

    <Human as Wizard>::power();
    Human::power();

    String::from("hello").outline_print();

    println!(
        "{}",
        DisplayableVec(vec![String::from("hello"), String::from("world")])
    );

    println!("do_twice(add_one, 4) = {}", do_twice(add_one, 4));

    Human::hello_macro();
}

fn dangerous(a: *const i32, b: *mut i32) {
    unsafe {
        println!("a is: {}", *a);
        println!("b is: {}", *b);
    }
}

unsafe fn unsafe_dangerous(x: *const i32, y: *mut i32) {
    println!("x is: {}", *x);
    println!("y is: {}", *y);
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}
