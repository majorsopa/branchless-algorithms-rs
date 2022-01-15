// returns a non-zero integer if a is greater than b, and zero if b is greater or equal to a
pub fn greater_than(a: isize, b: isize) -> usize {
  (super::maximum::maximum(a, b) ^ b) as usize
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_greater_than() {
    assert_eq!(0, greater_than(1, 2));
    assert_eq!(0, greater_than(-3, 4));
    assert_eq!(0, greater_than(-6, -1));
    assert_eq!(0, greater_than(5, 5));

    assert_ne!(0, greater_than(6, 2));
    assert_ne!(0, greater_than(100, -5));
    assert_ne!(0, greater_than(-32, -600));
  }
}
