pub fn absolute_value_i32(a: i32) -> i32 {
  let mask = a >> 31;
  (mask ^ a) - mask
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn absolute_value() {
    assert!(absolute_value_i32(7) == 7);
    assert!(absolute_value_i32(-3) == 3);
  }
}
