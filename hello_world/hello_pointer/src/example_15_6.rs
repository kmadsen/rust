// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html


use List::{Cons, Nil};
use std::rc::Rc;
use core::cell::RefCell;

#[derive(Debug)]
enum List {
  Cons(i32, RefCell<Rc<List>>),
  Nil
}

impl List {
  fn tail(&self) -> Option<&RefCell<Rc<List>>> {
    match self {
      Cons(_, item) => Some(item),
      Nil => None
    }
  }
}

#[allow(dead_code)]
pub fn demonstrate_memory_leak() {
  println!("hello");

  let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

  println!("a initial rc count: {}", Rc::strong_count(&a));
  println!("a next item: {:?}", a.tail());

  let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

  println!("a rc count after b clone: {}", Rc::strong_count(&a));
  println!("b initial rc count: {}", Rc::strong_count(&b));
  println!("b next item: {:?}", b.tail());

  if let Some(link) = a.tail() {
    *link.borrow_mut() = Rc::clone(&b);
  }

  // Notice the reference count is 2 instead of 1. So the memory allocated
  // on the heap will not be cleaned up after this exits.
  println!("a rc count after changing a: {}", Rc::strong_count(&a));
  println!("b rc count after changing a: {}", Rc::strong_count(&b));
}

/**
 * This section below demonstrates how to make a tree without causing
 * memory leaks.
 */

use std::rc::{Weak};

#[derive(Debug)]
struct Node {
  value: i32,
  parent: RefCell<Weak<Node>>,
  children: RefCell<Vec<Rc<Node>>>
}

#[allow(dead_code)]
pub fn deomonstrate_closed_graph() {
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

  let branch = Rc::new(Node {
    value: 5,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![Rc::clone(&leaf)]),
  });

  *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

  // Notice there is not an infinite loop, the tree is
  println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
}

pub fn run() {
  let leaf = Rc::new(Node {
    value: 3,
    parent: RefCell::new(Weak::new()),
    children: RefCell::new(vec![]),
  });

  println!(
    "leaf strong = {}, weak = {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf),
  );

  {
    let branch = Rc::new(Node {
      value: 5,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
      "branch strong: {}, weak: {}",
      Rc::strong_count(&branch),
      Rc::weak_count(&branch)
    );

    println!(
      "leaf strong: {}, weak: {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf)
    );
  }

  println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
  println!(
    "leaf strong: {}, weak: {}",
    Rc::strong_count(&leaf),
    Rc::weak_count(&leaf)
  );
}
