pub fn is_prime(num: usize) -> bool {
  let mut prime = Vec::new();
  prime.push(!(num <= 1 || num % 2 == 0 || num % 3 == 0));

  let mut i = 5;
  while i * i <= num {
    prime.push(!(num % i == 0 || num % (i + 2) == 0));
    i += 6;
  }
  
  ((num == 2 || num == 3) || !prime.contains(&false)) && num != 0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_is_prime() {
    assert!(is_prime(2));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(227));

    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(100));
  }
}
