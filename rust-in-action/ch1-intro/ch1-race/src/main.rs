// Race condition

use std::thread;

fn main() {
    let mut data = 100;

    // thread::spawn() takes a closure
    // Error - cannot borrow `data` as mutable more than once at a time
    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        // Error - second mutable borrow occurs here
        data = 1000;
    });
    println!("{}", data);
}
