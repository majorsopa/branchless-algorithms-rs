pub fn bubble_sort(arr: &mut [i32]) {
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
    let mut vec = vec![3, 2, 1];
    bubble_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }

  #[test]
  fn ascending() {
    let mut vec = vec![1, 2, 3];
    bubble_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }

  #[test]
  fn mixed() {
    let mut vec = vec![2, 1, 3];
    bubble_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }

  #[test]
  fn duplicates() {
    let mut vec = vec![2, 2, 3];
    bubble_sort(&mut vec);
    for i in 1..vec.len() {
      assert!(vec[i - 1] <= vec[i]);
    }
  }
}
