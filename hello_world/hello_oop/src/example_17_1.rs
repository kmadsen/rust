// https://doc.rust-lang.org/book/ch17-01-what-is-oo.html

#[derive(Debug)]
pub struct AveragedCollection {
  list: Vec<i32>,
  average: f64,
}

impl AveragedCollection {
  pub fn add(&mut self, value: i32) -> &mut Self {
    self.list.push(value);
    self.update_average();
    self
  }

  pub fn remove(&mut self) -> Option<i32> {
    let result = self.list.pop();
    match result {
      Some(value) => {
        self.update_average();
        Some(value)
      }
      None => None,
    }
  }

  pub fn average(&self) -> f64 {
    self.average
  }

  fn update_average(&mut self) -> &mut Self {
    let total: i32 = self.list.iter().sum();
    self.average = total as f64 / self.list.len() as f64;
    self
  }
}

#[allow(dead_code)]
pub fn run() {
  let mut averaged = AveragedCollection {
    list: vec![],
    average: 0.0,
  };

  averaged.add(10).add(31).add(-12);
  println!("What's the average: {:?}", averaged);

  averaged.remove();
  println!("Removed one: {:?}", averaged);

  println!("Just read the average: {}", averaged.average());
}
