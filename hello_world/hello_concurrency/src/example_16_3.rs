// https://doc.rust-lang.org/book/ch16-03-shared-state.html

use std::sync::Mutex;
use std::thread;

#[allow(dead_code)]
pub fn shared_state() {
  let m = Mutex::new(5);

  {
    let mut num = m.lock().unwrap();
    *num = 6
  }

  println!("m = {:?}", m);
}

use std::sync::Arc;

#[allow(dead_code)]
pub fn shared_state_across_threads() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
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

#[allow(dead_code)]
pub fn run() {
  println!("# run shared_state");
  shared_state();

  println!("# run shared_state_across_threads");
  shared_state_across_threads();
}
