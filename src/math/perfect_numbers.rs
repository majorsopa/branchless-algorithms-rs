pub fn is_perfect_number(num: usize) -> usize {
  let mut sum = 1;
  for i in 2..num {
    sum += i * (super::zero_checks::check_non_zero_unsigned(num % i) ^ 1);
  }
  sum ^ num
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check() {
    assert_eq!(is_perfect_number(6), 0);
    assert_eq!(is_perfect_number(28), 0);
    assert_eq!(is_perfect_number(496), 0);
    assert_eq!(is_perfect_number(8128), 0);

    assert_ne!(is_perfect_number(0), 0);
    assert_ne!(is_perfect_number(8), 0);
    assert_ne!(is_perfect_number(16), 0);
    assert_ne!(is_perfect_number(88), 0);
  }
}
