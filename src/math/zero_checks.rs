fn checks(num: &mut usize) {
  let mut num_bits = std::mem::size_of::<usize>() * 8;
  while num_bits > 0 {
    num_bits = num_bits >> 1;
    *num |= *num >> num_bits;
  }
}

pub fn check_non_zero_unsigned(num: usize) -> usize {  // returns 1 if non-zero, and 0 if zero
  let mut num = num;
  checks(&mut num);
  num & 1
}

pub fn check_zero_unsigned(num: usize) -> usize {  // returns 0 if non-zero, and 1 if zero
  let mut num = num;
  checks(&mut num);
  (!num) & 1
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_non_zero() {
    assert_eq!(check_non_zero_unsigned(0), 0);

    assert_eq!(check_non_zero_unsigned(1), 1);
    assert_eq!(check_non_zero_unsigned(12), 1);
    assert_eq!(check_non_zero_unsigned(8128), 1);
  }

  #[test]
  fn test_zero() {
    assert_eq!(check_zero_unsigned(0), 1);

    assert_eq!(check_zero_unsigned(1), 0);
    assert_eq!(check_zero_unsigned(12), 0);
    assert_eq!(check_zero_unsigned(8128), 0);
  }
}
