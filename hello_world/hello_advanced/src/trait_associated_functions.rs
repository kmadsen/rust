#[allow(dead_code)]
pub fn run() {
  traits_with_same_function_names();
  impls_with_same_function_names();
}

trait Pilot {
  fn fly(&self);
}

trait Wizard {
  fn fly(&self);
}

struct Human;

impl Pilot for Human {
  fn fly(&self) {
    println!("This is your captain speaking.");
  }
}

impl Wizard for Human {
  fn fly(&self) {
    println!("Up!");
  }
}

impl Human {
  fn fly(&self) {
    println!("*waving arms furiously*");
  }
}

#[allow(dead_code)]
pub fn traits_with_same_function_names() {
  let person = Human;
  Pilot::fly(&person);
  Wizard::fly(&person);
  person.fly();
}

trait Animal {
  fn baby_name() -> String;
}

struct Dog;

#[allow(dead_code)]
impl Dog {
  fn baby_name() -> String {
    String::from("Spot")
  }
}

impl Animal for Dog {
  fn baby_name() -> String {
    String::from("puppy")
  }
}

#[allow(dead_code)]
pub fn impls_with_same_function_names() {
  println!("A dog name could be {}", Dog::baby_name());
  println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}
