// https://doc.rust-lang.org/book/ch16-01-threads.html

use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn simple_example() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("hi number {} from the spawned thread", i)
    }
  });

  for i in 1..5 {
    println!("hi number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }

  handle.join().unwrap();
}

#[allow(dead_code)]
fn move_to_thread() {
  let v = vec![1, 2, 3];

  let handle = thread::spawn(move || {
    println!("Vector moved into the thread: {:?}", v);
  });

  // Does not compile: ownership has been moved to the thread scope
  // println!("Try access vector: {:?}", v);

  handle.join().unwrap()
}

#[allow(dead_code)]
pub fn run() {
  simple_example();
  move_to_thread();
}