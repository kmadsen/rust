mod patterns;

struct Point {
  x: i32,
  y: i32,
}

#[allow(dead_code)]
fn destructure_match() {
  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
  }
}

#[allow(dead_code)]
fn desctructure_and_combine_match() {
  let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
  println!("{} {} {} {}", feet, inches, x, y);
}

#[allow(dead_code)]
fn ignoring_values_and_default_match() {
  let mut setting_value = None;
  let new_setting_value = Some(10);

  match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
      println!("Can't overwrite an existing customized value");
    }
    _ => {
      setting_value = new_setting_value;
    }
  }
  println!("setting is {:?}", setting_value);
}

#[allow(dead_code)]
fn conditional_match() {
  let num = Some(4);

  match num {
    Some(x) if x < 5 => println!("less than five: {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }
}

#[allow(dead_code)]
fn expression_match() {
  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("y yes"),
    _ => println!("y no"),
  }

  match x {
    4 | 5 | 6 if !y => println!("!y yes"),
    _ => println!("!y no"),
  }
}

#[allow(dead_code)]
fn at_operator_in_match() {
  enum Message {
    Hello { id: i32 },
  }

  let message = Message::Hello { id: 3 };

  match message {
    // id is not visible in the arm because the conditional range
    // makes it unknown. Assign id_variable with the @ operator.
    Message::Hello {
      id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable),
    // id is not available in this arm.
    Message::Hello { id: 10..=12 } => {
      println!("Found an id in another range")
    }
    // id is available here with the struct shorthand syntax.
    Message::Hello { id } => println!("Found some other id: {}", id),
  }
}

fn main() {
  patterns::run();

  destructure_match();
  desctructure_and_combine_match();
  ignoring_values_and_default_match();
  conditional_match();
  expression_match();
  at_operator_in_match()
}
