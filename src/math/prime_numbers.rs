pub fn num_factors(num: usize) -> usize {  // returns the number of factors `num` has
  use super::zero_checks::check_zero_unsigned;
  let mut ret_value = 0;
  for i in 1..=num {
    ret_value += check_zero_unsigned(num % i);
  }
  ret_value
}

pub fn check_prime(num: usize) -> usize {  // returns 1 if prime and 0 if not
  super::inequalities::equal_to(2, num_factors(num) as isize)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn check_is_prime() {
    assert_eq!(1, check_prime(2));
    assert_eq!(1, check_prime(11));
    assert_eq!(1, check_prime(77761));

    assert_eq!(0, check_prime(0));
    assert_eq!(0, check_prime(1));
    assert_eq!(0, check_prime(77));
  }

  #[test]
  fn check_num_factors() {
    assert_eq!(2, num_factors(2));
    assert_eq!(2, num_factors(5));
    assert_eq!(2, num_factors(7));
    assert_eq!(2, num_factors(11));
    assert_eq!(2, num_factors(227));
    assert_eq!(2, num_factors(353));

    assert_eq!(0, num_factors(0));
    assert_eq!(1, num_factors(1));
    assert_eq!(3, num_factors(4));
    assert_eq!(4, num_factors(6));
    assert_eq!(8, num_factors(40));
    assert_eq!(4, num_factors(77));
    assert_eq!(9, num_factors(100));
  }
}
