use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let vector = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("vector = {:?}", vector);

        for i in 1..10 {
            println!("counting {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("counting {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi1"),
            String::from("hi2"),
            String::from("hi3"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("more"), String::from("messages")];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Received from thread: {}", received);
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("from thread {}, count is {}", i, *num);
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    deadlock();

    println!("Done!");
}

fn deadlock() {
    let m11 = Arc::new(Mutex::new(1));
    let m12 = Arc::clone(&m11);
    let m13 = Arc::clone(&m11);
    let m21 = Arc::new(Mutex::new(2));
    let m22 = Arc::clone(&m21);
    let m23 = Arc::clone(&m21);

    let h1 = thread::spawn(move || {
        println!("Locking num1 in thread 1...");
        let mut num1 = m11.lock().unwrap();
        *num1 = 11;

        println!("Sleeping in thread 1...");
        thread::sleep(Duration::from_secs(1));

        println!("Locking num2 in thread 1...");
        let mut num2 = m22.lock().unwrap();
        *num2 = 13;
    });
    let h2 = thread::spawn(move || {
        println!("Locking num2 in thread 2...");
        let mut num2 = m21.lock().unwrap();
        *num2 = 12;

        println!("Sleeping in thread 2...");
        thread::sleep(Duration::from_secs(1));

        println!("Locking num1 in thread 2...");
        let mut num1 = m12.lock().unwrap();
        *num1 = 14;
    });

    h1.join().unwrap();
    h2.join().unwrap();

    println!(
        "num 1 = {}, num2 = {}",
        m13.lock().unwrap(),
        m23.lock().unwrap()
    );
}
