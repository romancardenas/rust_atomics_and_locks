use rust_atomics_and_locks::*;

const N_THREADS: usize = 4;

fn main() {
    let mut handles = vec![]; // will contain all  the thread handles
    for _ in 0..N_THREADS {
        handles.push(thread::spawn(f));
    }
    println!("Hello from main thread!");
    for handle in handles {
        // wait for all threads to finish before exiting
        handle.join().unwrap();
    }
}

fn f() {
    let id = thread::current().id();
    println!("Hello from thread {id:?}!");
}
