pub fn minimum_i32_abs(a: i32, b: i32) -> i32 {
  use super::absolute_value::absolute_value_i32;
  (a + b - absolute_value_i32(a - b)) / 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn minimum() {
    assert!(12 == minimum_i32_abs(14, 12));
    assert!(-6 == minimum_i32_abs(-6, 3));
  }
}
