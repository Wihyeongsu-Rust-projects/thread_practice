use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

fn ex1() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // wait spwaned thread
    handle.join().unwrap();
}

fn ex2() {
    let v = vec![1, 2, 3];

    // thread must take ownership of v -> move
    let handle = thread::spawn(move || {
        //
        println!("Here's a vector {v:?}")
    });

    // wait spwaned threads
    handle.join().unwrap();
}

// Share memory by communication
fn ex3() {
    // open a channel
    let (tx, rx) = channel();

    // thread sends message using tx
    thread::spawn(move || {
        let msg = String::from("Hi");
        tx.send(msg).unwrap();
    });

    // receive message from tx
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn ex4() {
    let (tx, rx) = channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx as iterator terminated when channel is close
    for received in rx {
        println!("Got: {received}");
    }
}

fn ex5() {
    let (tx, rx) = channel();
    // create new tx
    let tx2 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // messages are sent non-deterministically
    for received in rx {
        println!("Got: {received}");
    }
}

fn ex6() {
    let m = Mutex::new(5);

    {
        // lock MutexGuard and get access to data
        let mut num = m.lock().unwrap();
        *num = 6;
        // automatically unlock MutexGuard when dropped
    }

    println!("m = {m:?}");
}

fn ex7() {
    // Arc smart pointer for threads get multiple ownership to counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // threads increase the counter
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap())
}

fn main() {
    ex7();
}
