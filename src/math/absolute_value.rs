pub fn absolute_value(a: i32) -> i32 {
  let mask = a >> 31;
  (mask ^ a) - mask
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn absolute_value_check() {
    assert!(absolute_value(7) == 7);
    assert!(absolute_value(-3) == 3);
  }
}
