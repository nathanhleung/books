use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum GenericList<T> {
    GenericCons(T, Box<GenericList<T>>),
    GenericNil,
}

enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum MutableRcList {
    MutableRcCons(Rc<RefCell<i32>>, Rc<MutableRcList>),
    MutableRcNil,
}

#[derive(Debug)]
enum CyclicList {
    CyclicCons(i32, RefCell<Rc<CyclicList>>),
    CyclicNil,
}

impl CyclicList {
    fn tail(&self) -> Option<&RefCell<Rc<CyclicList>>> {
        match self {
            CyclicCons(_, item) => Some(item),
            CyclicNil => None,
        }
    }
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping `CustomSmartPointer` with data `{}`!", self.data);
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

use crate::CyclicList::{CyclicCons, CyclicNil};
use crate::GenericList::{GenericCons, GenericNil};
use crate::List::{Cons, Nil};
use crate::MutableRcList::{MutableRcCons, MutableRcNil};
use crate::RcList::{RcCons, RcNil};

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
    println!();

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list = {:?}", list);

    let generic_list = GenericCons(
        "a",
        Box::new(GenericCons(
            "b",
            Box::new(GenericCons("c", Box::new(GenericNil))),
        )),
    );
    println!("generic_list = {:?}", generic_list);
    println!();

    let x = 5;
    let x_ref = &x;
    let x_box = Box::new(x);
    let x_mybox = MyBox::new(x);
    assert_eq!(x, *x_ref);
    assert_eq!(x, *x_box);
    assert_eq!(x, *x_mybox);
    println!(
        "x = {}, x_ref = {:p}, x_box = {:?}, x_mybox = {:?}",
        x, x_ref, x_box, x_mybox
    );

    hello("World");
    let world_boxed = MyBox::new("World");
    hello(&world_boxed);
    println!();

    let c1 = CustomSmartPointer {
        data: String::from("data 1"),
    };
    let c2 = CustomSmartPointer {
        data: String::from("data 2"),
    };
    let c3 = CustomSmartPointer {
        data: String::from("data 3"),
    };
    println!("c1 = {:?}, c2 = {:?}, c3 = {:?}", c1, c2, c3);
    println!("Dropping c1 early...");
    drop(c1);
    println!("Dropped c1!");
    println!();

    let a = Rc::new(RcCons(
        5,
        Rc::new(RcCons(10, Rc::new(RcCons(10, Rc::new(RcNil))))),
    ));
    println!(
        "reference count after creating a = {}",
        Rc::strong_count(&a)
    );
    let b = Rc::new(RcCons(3, Rc::clone(&a)));
    println!(
        "reference count after creating b = {}",
        Rc::strong_count(&a)
    );
    {
        let c = Rc::new(RcCons(4, Rc::clone(&a)));
        println!(
            "reference count after creating c = {}",
            Rc::strong_count(&a)
        );
    }
    println!(
        "reference count after c goes out of scope = {}",
        Rc::strong_count(&a)
    );
    println!();

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(MutableRcCons(Rc::clone(&value), Rc::new(MutableRcNil)));
    let b = MutableRcCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = MutableRcCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);
    println!();

    let a = Rc::new(CyclicCons(5, RefCell::new(Rc::new(CyclicNil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a.tail() = {:?}", a.tail());

    let b = Rc::new(CyclicCons(10, RefCell::new(Rc::clone(&a))));
    println!("a new rc count = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b.tail() = {:?}", b.tail());

    if let Some(a_tail) = a.tail() {
        // Create cycle
        *a_tail.borrow_mut() = Rc::clone(&b);
    }

    println!("a new rc count = {}", Rc::strong_count(&a));
    println!("b new rc count = {}", Rc::strong_count(&b));
    // Cause stack overflow
    // println!("a next item = {:?}", a.tail());
    println!();

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf.parent = {:?}, leaf strong = {}, leaf weak = {}",
        leaf.parent.borrow().upgrade(),
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "leaf strong = {}, leaf weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!(
            "branch strong = {}, branch weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
    }

    // Does not cause stack overflow
    println!(
        "leaf.parent = {:?}, leaf strong = {}, leaf weak = {}",
        leaf.parent.borrow().upgrade(),
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn double_borrow() {
        let cell = RefCell::new(5);
        let mut _borrow1 = cell.borrow_mut();
        let mut _borrow2 = cell.borrow_mut();
    }
}
