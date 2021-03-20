/**
 * This class shows crate basics. Unit tests are within the same
 * file as the implementation. This also means it can test private
 * entities.
 *
 * Examples:
 *  Run all tests with $ cargo test
 *  Run a single test with  $ cargo test sanity
 */

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub fn divide(top: i32, bottom: i32) -> f64 {
    top as f64 / bottom as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn test_private_functions() {
        let result = internal_adder(2, 2);

        assert_eq!(4, result);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        divide(2, 0);
    }

    #[test]
    fn divide_returns_floating_point() {
        let result = divide(10, 20);

        assert_eq!(0.5, result);
    }
}
