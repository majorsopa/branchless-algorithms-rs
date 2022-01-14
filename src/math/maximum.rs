pub fn maximum(a: isize, b: isize) -> isize {
  (a + b + super::absolute_value::absolute_value(a - b)) / 2
}

pub fn maximum_of_list(list: &[isize]) -> isize {
  let mut current_max = list[0];
  for i in 1..list.len() {
    current_max = maximum(current_max, list[i]);
  }
  current_max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn maximum_check() {
    assert_eq!(7, maximum(7, 3));
    assert_eq!(3, maximum(-6, 3));
    assert_eq!(4, maximum(4, 4));
  }

  #[test]
  fn maximum_list_check() {
    assert_eq!(7, maximum_of_list(&vec![1, 7, 3]));
    assert_eq!(100, maximum_of_list(&vec![1, 0, 100, -5]));
  }
}
