#[allow(dead_code)]
pub fn run() {
  run_my_vec_macro();
}

fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>())
}

// https://doc.rust-lang.org/reference/macros-by-example.html
// For more information on macro pattern syntax
// This is a simplified version of the vec! macro
#[macro_export]
macro_rules! my_vec {
  () => (
    Vec::new()
  );
  ( $( $x:expr ),* ) => {
    {
      let mut temp_vec = Vec::new();
      $(
        temp_vec.push($x);
      )*
      temp_vec
    }
  };
  ($elem:expr; $n:expr) => (
      std::vec::from_elem($elem, $n)
  );
}

fn run_my_vec_macro() {
  let v: Vec<u32> = my_vec![1, 2, 3];
  let first = v.iter().next().unwrap();
  print_type_of(first);

  let mapped: Vec<u32> = my_vec![].iter().map(|x| x + 1u32).collect();
  println!("{:?}", mapped);
}

#[test]
fn empty_vec_test() {
  let v: Vec<u32> = my_vec![];
  assert!(v.is_empty());
}

#[test]
fn values_vec_test() {
  let v = my_vec![10, 11, 12];
  assert_eq!(10, v[0]);
  assert_eq!(11, v[1]);
  assert_eq!(12, v[2]);
}

#[test]
fn intiaization_vec_test() {
  let v = my_vec![-4; 3];
  assert_eq!(v, [-4, -4, -4]);
}
