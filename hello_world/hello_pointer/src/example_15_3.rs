use once_cell::sync::Lazy;

static START_INSTANT: Lazy<std::time::Instant> = Lazy::new(|| {
  std::time::Instant::now()
});

// It would be good to be able to define this outside of this file.
// But I don't know how to do that yet, will figure it out later.
macro_rules! logi {
  ($($arg:tt)*) => ({
    let duration: std::time::Duration = START_INSTANT.elapsed();
    print!("{}.{}: ", duration.as_secs(), duration.subsec_nanos());
    println!($($arg)*)
  })
}

struct CustomSmartPointer {
  data: String,
}

impl Drop for CustomSmartPointer {
  fn drop(&mut self) {
    logi!("Dropping CustomSmartPointer with data {}!", self.data)
  }
}

#[allow(dead_code)]
pub fn run() {
  logi!("example_15_3 started");

  let c = CustomSmartPointer {
    data: String::from("my stuff"),
  };
  let _d = CustomSmartPointer {
    data: String::from("other stuff"),
  }; 
  logi!("CustomSmartPointers created.");
  
  // Explicit call to drop c. This forced c to be dropped before d
  // References are normally dropped in reverse order of creation.
  drop(c);
  logi!("CustomSmartPointers dropped before the end of main");
}
