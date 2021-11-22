/// https://doc.rust-lang.org/book/ch17-02-trait-objects.html
///
/// Create a gui framework
///
/// There is a small section talking about the performance impact
/// of dynamic dispatch. Rust needs to figure which object to call
/// at runtime, as opposed to having the type at compile time. Here
/// is an interesting read where a blogger ran some tests
/// https://medium.com/digitalfrontiers/rust-dynamic-dispatching-deep-dive-236a5896e49b
/// tl;dr it can be up to 3x slower, but also have little to no impact
/// when you give the compiler a chance to figure out the type.

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub text: String,
}

impl Draw for Button {
  fn draw(&self) {
    println!("draw button");
  }
}

pub struct SelectBox {
  pub width: u32,
  pub height: u32,
  pub options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    println!("draw select box");
  }
}

#[allow(dead_code)]
pub fn run() {
  let screen = Screen {
    components: vec![
      Box::new(SelectBox {
        width: 75,
        height: 10,
        options: vec![
          String::from("Yes"),
          String::from("Maybe"),
          String::from("No"),
        ],
      }),
      Box::new(Button {
        width: 50,
        height: 10,
        text: String::from("Ok"),
      }),
    ],
  };

  screen.run();
}
