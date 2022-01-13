pub fn selection_sort(arr: &mut [i32]) {
  for i in 0..arr.len() {
    let mut smallest = i;
    for b in (i + 1)..arr.len() {
      let condition = usize::from(arr[b] < arr[smallest]);
      let rev_condition = usize::from(!(arr[b] < arr[smallest]));
      smallest = b * condition + smallest * rev_condition;
    }
    arr.swap(smallest, i);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn descending() {
    let mut vec = vec![3, 2, 1];
    selection_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }

  #[test]
  fn ascending() {
    let mut vec = vec![-4, 2, 3, 4, 7, 10];
    selection_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }

  #[test]
  fn duplicates() {
    let mut vec = vec![-4, -4, 3, 7, 4, 15];
    selection_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }
}
