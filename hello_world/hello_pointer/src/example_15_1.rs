// https://doc.rust-lang.org/book/ch15-01-box.html

/**
 * Below is an example of a recursive type. Using Box
 * makes it so the recursive size is not infinite.
 */

#[derive(Debug)]
enum List {
  Cons(i32, Box<List>),
  Nil,
}

use List::{Cons, Nil};


#[allow(dead_code)]
pub fn run() {
  println!("b = {}", Box::new(5));

  let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  println!("list = {:?}", list);
}
