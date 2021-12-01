#[allow(dead_code)]
pub fn run() {
  dereferencing_raw_pointers();
  calling_unsafe_functions();
  creating_safe_over_unsafe();
  run_external_code();
  mutable_global_static_variable();
}

#[allow(dead_code)]
fn dereferencing_raw_pointers() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    // Referencing and changing values from raw pointers
    // requires an unsafe block. As you can see this allows
    // you to ignore some borrowing philosphies. Possible optimization?
    println!("before r1 is: {}", *r1);
    println!("before r2 is: {}", *r2);
    *r2 = 8;
    println!("after r1 is: {}", *r1);
    println!("after r2 is: {}", *r2);
  }
}

#[allow(dead_code)]
unsafe fn unsafe_function() {
  println!("called the unsafe_function")
}

fn calling_unsafe_functions() {
  unsafe {
    unsafe_function();
  }
}

#[allow(dead_code)]
fn creating_safe_over_unsafe() {
  // This won't work because you're borrowing the same array twice
  // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  //   let len = slice.len();
  //   assert!(mid <= len);
  //   (&mut slice[..mid], &mut slice[mid..])
  // }

  use std::slice;
  fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
      )
    }
  }

  let mut v = vec![1, 2, 3, 4, 5, 6];
  let r = &mut v[..];
  // Existing rust version 
  // let (a, b) = r.split_at_mut(3);

  // Testing the above solution. The mut vec r is safe because the 
  // borrow checker helps ensure a single vec is mutable.
  let (a, b) = split_at_mut(r, 3);

  assert_eq!(a, &mut [1, 2, 3]);
  assert_eq!(b, &mut [4, 5, 6]);
}

#[allow(dead_code)]
fn run_external_code() {
  unsafe {
    println!("Absolute value of -3 according to C: {}", abs(-3));
  }
}

extern "C" {
  fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c() {
  println!("Just called a Rust function from C!");
}

#[allow(dead_code)]
fn mutable_global_static_variable() {
  add_to_count(12);
  add_to_count(4);
}

static mut GLOBAL_STATIC_COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
  unsafe {
    let old_value = GLOBAL_STATIC_COUNTER;
    GLOBAL_STATIC_COUNTER += inc;
    println!("add_to_count {} is now {}", old_value, GLOBAL_STATIC_COUNTER);
  }
}

unsafe trait Foo {
  // methods go here
}

unsafe impl Foo for i32 {
  // method implementations go here
}
