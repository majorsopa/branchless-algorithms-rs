pub fn is_perfect_number(num: usize) -> bool {
  let num_bits = std::mem::size_of::<usize>() * 8;
  let mut sum = 1;
  for i in 2..num {
    sum += i * (super::check_non_zero_unsigned::check_non_zero_unsigned(num % i) ^ 1);
  }
  (sum ^ num) == 0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check() {
    assert!(is_perfect_number(6));
    assert!(is_perfect_number(28));
    assert!(is_perfect_number(496));
    assert!(is_perfect_number(8128));

    assert!(!is_perfect_number(0));
    assert!(!is_perfect_number(8));
    assert!(!is_perfect_number(16));
    assert!(!is_perfect_number(88));
  }
}
