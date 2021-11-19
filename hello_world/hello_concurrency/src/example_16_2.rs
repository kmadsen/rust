// https://doc.rust-lang.org/book/ch16-02-message-passing.html

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub fn simple_send_recv() {
  let (tx, rx) = mpsc::channel();

  thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();

    // Val lost ownership, it has been passed through send
    // println!("val is {}", val);
  });

  let received = rx.recv().unwrap();
  println!("Received: {}", received);
}

#[allow(dead_code)]
pub fn open_connection() {
  let (tx, rx) = mpsc::channel();

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
      // End of scope triggers the end of the rx iterator
  });

  for received in rx {
      println!("Received: {}", received);
  }
  // Notice the end is None
  // let mut iter = rx.iter();
  // assert_eq!(iter.next(), Some("hi".to_string()));
  // assert_eq!(iter.next(), Some("from".to_string()));
  // assert_eq!(iter.next(), Some("the".to_string()));
  // assert_eq!(iter.next(), Some("thread".to_string()));
  // assert_eq!(iter.next(), None);
}

#[allow(dead_code)]
pub fn multiple_senders() {
  let (tx, rx) = mpsc::channel();

  // Create another sender for the next thread
  // before passing the ownership to the current thread
  let tx1 = tx.clone();
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
      tx1.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for received in rx {
    println!("Received: {}", received);
  }
}

#[allow(dead_code)]
pub fn run() {
  println!("# run simple_send_recv");
  simple_send_recv();

  println!("# run open_connection");
  open_connection();
  
  println!("# run multiple_senders");
  multiple_senders();
}
