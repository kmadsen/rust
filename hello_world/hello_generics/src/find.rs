pub fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
      if item > largest {
          largest = item
      }
  }

  largest
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shoud_find_largest_integer() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    assert_eq!(100, *result, "expected {} but was {}", 100, result);
  }

  #[test]
  fn shoud_find_largest_character() {
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    assert_eq!('y', *result, "expected {} but was {}", 'y', result);
  }
}
