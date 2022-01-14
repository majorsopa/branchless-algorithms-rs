pub fn absolute_value(a: isize) -> isize {
  let mask = a >> std::mem::size_of::<isize>();
  (mask ^ a) - mask
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn absolute_value_check() {
    assert_eq!(absolute_value(7), 7);
    assert_eq!(absolute_value(-3), 3);
  }
}
