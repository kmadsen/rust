// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

pub trait Messanger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messanger> {
  messenger: &'a T,
  value: usize,
  max: usize
}

#[allow(dead_code)]
impl<'a, T> LimitTracker<'a, T>
where
  T: Messanger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("Error: You are over your quota!");
    } else if percentage_of_max > 0.9 {
      self.messenger.send("Urgent warning: You've used over 90% of your quota!");
    } else if percentage_of_max > 0.75 {
      self.messenger.send("Warning: You've used over 75% of your quota!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger  {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger  {
    fn new() -> MockMessenger  {
      MockMessenger  {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messanger for MockMessenger  {
    fn send(&self, msg: &str) {
      self.sent_messages.borrow_mut().push(String::from(msg));

      // Below is an example of a runtime panic!, cannot have multiple
      // barrowed references in the same scope. This can be fixed by 
      // narrowing the scope. But major warning of a runtime crash! We
      // prefer compile time errors, but RefCell is okay for tests.
      // let mut one_borrow = self.sent_messages.borrow_mut();
      // let mut two_borrow = self.sent_messages.borrow_mut();

      // one_borrow.push(String::from(message));
      // two_borrow.push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  } 
}

// https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt

#[derive(Debug)]
enum List {
  Cons(Rc<RefCell<i32>>, Rc<List>),
  Nil,
}

use List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[allow(dead_code)]
pub fn run() {
  let value = Rc::new(RefCell::new(6));
  let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
  
  let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
  let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
  let d = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));
  
  println!("a after = {:?}", a);
  // println!("b after = {:?}", b);

  // This isn't from the book, but here's an example of
  // destructuring the enum
  if let List::Cons(value, next) = b {
    print!("b: {} next: {:?}", &value.borrow(), next);
  }

  // Also not from the book, but here's an example of
  // pattern matching. But these feels wrong considering we know the type.
  match c {
    List::Cons(value, next) => println!("c value:{} next:{:?}", value.borrow(), next),
    _ => println!("Did not destructure the type")
  }

  println!("d after = {:?}", d);
}