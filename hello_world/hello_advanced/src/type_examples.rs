#[allow(dead_code)]
pub fn run() {
  synonym_for_i32();
  example_alias_for_long_type();
  // example_never_type(); Shows the -> ! never return.
  dynamically_sized_types();
}

fn synonym_for_i32() {
  type Kilometers = i32;
  let x: i32 = 5;
  let y: Kilometers = 5;

  println!("x + y = {}", x + y);
}

fn example_alias_for_long_type() {
  type Thunk = Box<dyn Fn() + Send + 'static>;
  let f: Thunk = Box::new(|| println!("hi box"));
  fn takes_long_type(thunk: Thunk) {
    thunk()
  }

  fn returns_long_type() -> Thunk {
    Box::new(|| println!("hi from function"))
  }

  takes_long_type(f);
  returns_long_type()();
}

#[allow(dead_code)]
fn example_never_type() -> ! {
  // -> ! means, it never returns.
  loop {
    println!("forever loop");
  }
}

fn dynamically_sized_types() {
  // https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait
  // Interesting section of the book because it describes how strings work aka str
  // This won't compile, because the size is unknown until runtime.
  // let s1: str = "Hello there!";
  // let s2: str = "How's it going?";

  // But this will compile, because there is a reference.
  // The reference points to a usize, which tells you the info you need for a string.
  // Neat!
  let s1: &str = "Hello there!";
  println!("{}", s1);

  // These two are essentially the same, the generic type expects to have a Sized
  fn generic<T>(_: T) {}
  fn generic_sized<T: Sized>(_: T) {}

  // To relax the default behavior, you can use this special syntax.
  fn generic_relaxed_sized<T: ?Sized>(_: &T) {}

  generic("generic");
  generic_sized("generic_sized");
  generic_relaxed_sized("generic_relaxed_sized");
}
