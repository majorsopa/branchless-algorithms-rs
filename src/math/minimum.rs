pub fn minimum(a: isize, b: isize) -> isize {
  (a + b - super::absolute_value::absolute_value(a - b)) / 2
}

pub fn minimum_of_list(list: &[isize]) -> isize {
  let mut current_min = list[0];
  for i in 1..list.len() {
    current_min = minimum(current_min, list[i]);
  }
  current_min
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn minimum_check() {
    assert_eq!(12, minimum(14, 12));
    assert_eq!(-6, minimum(-6, 3));
  }

  #[test]
  fn minimum_list_check() {
    assert_eq!(1, minimum_of_list(&vec![1, 7, 3]));
    assert_eq!(-5, minimum_of_list(&vec![1, 0, 100, -5]));
  }
}
