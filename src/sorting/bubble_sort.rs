pub fn bubble_sort(arr: &mut[i32]) {
  // small to large
  for i in 0..arr.len() {
    for b in 0..arr.len() - 1 - i {
      arr.swap(b, b + usize::from(arr[b] > arr[b + 1]))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn descending() {
    let mut ve1 = vec![3, 2, 1];
    bubble_sort(&mut ve1);
    for i in 1..ve1.len() {
      assert!(ve1[i - 1] <= ve1[i]);
    }
  }

  #[test]
  fn ascending() {
    let mut ve2 = vec![1, 2, 3];
    bubble_sort(&mut ve2);
    for i in 1..ve2.len() {
      assert!(ve2[i - 1] <= ve2[i]);
    }
  }
}
