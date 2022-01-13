pub fn maximum_i32_abs(a: i32, b: i32) -> i32 {
  use super::absolute_value::absolute_value_i32;
  (a + b + absolute_value_i32(a - b)) / 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maximum() {
    assert!(7 == maximum_i32_abs(7, 3));
    assert!(3 == maximum_i32_abs(-6, 3));
  }
}
