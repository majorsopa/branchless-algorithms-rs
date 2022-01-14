pub fn check_non_zero_unsigned(num: usize) -> usize {  // returns 1 if non-zero, and 0 if zero
  let mut num = num;
  let mut num_bits = std::mem::size_of::<usize>() * 8;
  while num_bits > 0 {
    num_bits /= 2;
    num |= num >> num_bits;
  }
  num & 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check() {
    assert_eq!(check_non_zero_unsigned(0), 0);

    assert_eq!(check_non_zero_unsigned(1), 1);
    assert_eq!(check_non_zero_unsigned(12), 1);
    assert_eq!(check_non_zero_unsigned(8128), 1);
  }
}
