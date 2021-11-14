// https://doc.rust-lang.org/book/ch15-02-deref.html

/**
 * Below is an example of our own Box implementation.
 * In order to dereference the MyBox type, you must implement
 * the Deref trait.
 */
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}

impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

fn hello(name: &str) {
  println!("Hello, {}!", name);
}

#[allow(dead_code)]
pub fn run() {
  let x = 5;
  let y = MyBox::new(x);

  assert_eq!(5, x);
  assert_eq!(5, *y);

  let m = MyBox::new(String::from("Rust"));
  hello(&m);
}
