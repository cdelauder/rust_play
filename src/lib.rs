
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
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