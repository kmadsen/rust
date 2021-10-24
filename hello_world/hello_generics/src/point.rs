
pub struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  pub fn x(&self) -> &T {
      &self.x
  }

  pub fn y(&self) -> &T {
      &self.y
  }
}

impl Point<f32> {
  pub fn magnitude(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}

impl Point<i32> {
  pub fn magnitude(&self) -> f32 {
      ((self.x.pow(2) + self.y.pow(2)) as f32).sqrt()
  }
}

impl std::cmp::PartialEq for Point<f32> {
  fn eq(&self, other: &Self) -> bool {
    self.x.eq(&other.x) && self.y.eq(&other.y)
  }
}

impl std::cmp::PartialOrd for Point<f32> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.magnitude().partial_cmp(&other.magnitude())
  }
}

impl std::cmp::PartialEq for Point<i32> {
  fn eq(&self, other: &Self) -> bool {
    self.x.eq(&other.x) && self.y.eq(&other.y)
  }
}

impl std::cmp::PartialOrd for Point<i32> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.magnitude().partial_cmp(&other.magnitude())
  }
}

impl<T : std::fmt::Display> Point<T> {
  pub fn json(&self) -> std::string::String {
      format!("{{x:{},y:{}}}", self.x, self.y)
  }
}

impl <T : std::fmt::Display> std::fmt::Display for Point<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}", self.json())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! assert_eq_delta {
    ($x:expr, $y:expr, $d:expr) => {
      let value = ($x - $y).abs(); 
      if (value > $d) { panic!("Expected {} but was {}", $x, $y); }
    }
  }

  #[test]
  fn veify_x_y() {
    let integer = Point { x: 5, y: 10 };

    assert_eq!(5, *integer.x());
    assert_eq!(10, *integer.y());
  }

  #[test]
  fn veify_integer_json() {
    let integer = Point { x: 5, y: 10 };

    assert_eq!("{x:5,y:10}", integer.json());

    println!("ijson {}", integer.json());
  }

  #[test]
  fn veify_float_json() {
    let float = Point { x: 5.1, y: 10.89 };

    assert_eq!("{x:5.1,y:10.89}", float.json());
  }

  #[test]
  fn veify_integer_magnitude() {
    let integer = Point { x: 500, y: 123 };

    assert_eq_delta!(514.9068, integer.magnitude(), 0.0001);
  }

  #[test]
  fn veify_float_magnitude() {
    let float = Point { x: 5.1, y: 10.89 };

    assert_eq_delta!(12.025, float.magnitude(), 0.0001);
  }

  #[test]
  fn veify_float_equality() {
    let float_lhs = Point { x: 5.1, y: 10.89 };
    let float_rhs = Point { x: 5.1, y: 10.89 };

    assert_eq!(true, float_lhs == float_rhs)
  }

  #[test]
  fn veify_integer_inequality() {
    let float_lhs = Point { x: 1, y: 2 };
    let float_rhs = Point { x: -1, y: -2 };

    assert_eq!(true, float_lhs != float_rhs)
  }

  #[test]
  fn veify_float_magnitude_compare() {
    let float_lhs = Point { x: 1, y: 2 };
    let float_rhs = Point { x: -1, y: -4 };

    assert_eq!(true, float_lhs < float_rhs)
  }
}
