#[allow(dead_code)]
pub fn run() {
  function_pointer();
  mapper_functions();

  // https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes
  // ^ current section
}

fn function_pointer() {
  fn add_one(x: i32) -> i32 {
    x + 1
  }

  fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
  }

  let answer = do_twice(add_one, 5);
  println!("Function pointers, do_twice(add_one, 5): {}", answer);
}

fn mapper_functions() {
  #[derive(Debug)]
  enum Status {
    Value(u32),
  }
  let list_of_statuses: Vec<Status> = (0u32..8).map(Status::Value).collect();
  let status_descructure = |&Status::Value(value)| value;
  let list_of_u32: Vec<u32> = list_of_statuses.iter().map(status_descructure).collect();
  println!("{:?}", list_of_u32);

  #[allow(dead_code)]
  fn status_mapper() -> Box<dyn Fn(Status) -> u32> {
    Box::new(|Status::Value(value)| value)
  }
}
