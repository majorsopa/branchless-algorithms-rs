pub fn is_perfect_number(num: usize) -> bool {
  let mut sum = 0;
  for i in 1..num {
    sum += i * usize::from(num % i == 0);
  }
  num == sum && !(num == 0)
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
