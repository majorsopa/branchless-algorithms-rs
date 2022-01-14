pub fn is_perfect_number(num: isize) -> bool {
  let num_bits = std::mem::size_of::<usize>() * 8;
  let mut sum = 0;
  for i in 1..num {
    let eee = (super::minimum::minimum(num % i, 1) ^ 1) & 1;
    println!("{},{}", num % i, eee);
    sum += i * (!(num % i) >> num_bits - 1)
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
    assert!(!is_perfect_number(-4));
    assert!(!is_perfect_number(-6));
  }
}
