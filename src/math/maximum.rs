pub fn maximum(a: i32, b: i32) -> i32 {
  (a + b + super::absolute_value::absolute_value(a - b)) / 2
}

pub fn maximum_of_list(list: &[i32]) -> i32 {
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
    assert!(7 == maximum(7, 3));
    assert!(3 == maximum(-6, 3));
  }

  #[test]
  fn maximum_list_check() {
    assert!(7 == maximum_of_list(&vec![1, 7, 3]));
    assert!(100 == maximum_of_list(&vec![1, 0, 100, -5]));
  }
}
