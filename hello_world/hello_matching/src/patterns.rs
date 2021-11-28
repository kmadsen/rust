fn run_else_if() {
  let favorite_color: Option<&str> = None;
  let is_tuesday = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(color) = favorite_color {
    println!("Using your favorite color, {}, as the background", color);
  } else if is_tuesday {
    println!("Tuesday is green day!");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Using purple as the background color");
    } else {
      println!("Using orange as the background color");
    }
  } else {
    println!("Using blue as the background color");
  }
}

fn run_while_let() {
  let mut stack = Vec::new();

  stack.push(1);
  stack.push(2);
  stack.push(3);

  while let Some(top) = stack.pop() {
    println!("{}", top);
  }
}

fn run_enumerate() {
  let v = vec!['a', 'b', 'c'];

  for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
  }
}

fn run_parameters() {
  let point = (37.80621024361224, -122.25859921446396);
  print_coordiantes(&point);
}

fn print_coordiantes(&(lat, lng): &(f64, f64)) {
  println!("Current location: ({}, {})", lat, lng);
}

#[allow(dead_code)]
pub fn run() {
  run_else_if();
  run_while_let();
  run_enumerate();
  run_parameters()
}
