// The reference counted smart pointer

use List::{Cons, Nil};
use std::rc::Rc;

enum List {
  Cons(i32, Rc<List>),
  Nil
}

// Simple reference counting example.
#[allow(dead_code)]
pub fn run() {
  let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
  println!("count after creating a = {}", Rc::strong_count(&a));
  let _b = Cons(3, Rc::clone(&a));
  println!("count after creating b = {}", Rc::strong_count(&a));
  {
    let _c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
  }
  println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
