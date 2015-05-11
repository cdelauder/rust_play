/// this is the function we are testing
pub fn add_two(a: i32) -> i32 {
    a + 2
}

/// this is how to build a test module
#[cfg(test)]
mod tests {
  /// pull in all of the public functions
  use super::*;
  #[test]
  fn test_two() {
    assert_eq!(4, add_two(2));
  }

  #[test]
  #[should_panic(expected="assertion failed")]
  fn it_works() {
      assert_eq!("Hello", "world");
  }
}