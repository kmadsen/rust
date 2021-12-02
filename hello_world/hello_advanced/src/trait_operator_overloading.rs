use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
  type Output = Millimeters;

  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}

/**
 * Note this will not compile. You may want to change the RHS of the
 * override operation, but there are more details here
 * https://github.com/rust-lang/rfcs/blob/master/text/0048-traits.md
 */
// impl Add<Millimeters> for Meters {
//   type Output = Meters;
//   fn add(self, other: Millimeters) -> Millimeters {
//       Millimeters(self.0 + (other.0 * 1000))
//   }
// }
// main() {
//   assert_eq!(
//     Meters(3) + Millimeters(112),
//     Millimeters(3112),
//   );
// }

#[allow(dead_code)]
pub fn run() {
  assert_eq!(
    Point { x: 1, y: -2 } + Point { x: 3, y: 4 },
    Point { x: 4, y: 2 }
  );
  assert_eq!(Millimeters(112) + Meters(3), Millimeters(3112),);
  println!("Completed the operator examples")
}
