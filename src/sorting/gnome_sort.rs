pub fn gnome_sort(arr: &[i32]) -> Vec<i32> {
  let mut arr = arr.to_vec();
  let mut i = 1;
  let mut b = 2;

  while i < arr.len() {
    let focus = arr[i];
    let comparison = arr[i - 1];
    let compared_usize = usize::from(comparison < focus);
    let reversed_compared_usize = usize::from(!(comparison < focus));


    {  // if comparison < focus
      i = b * compared_usize + i * reversed_compared_usize;
      b = (i + 1) * compared_usize + b * reversed_compared_usize;
    }
    {  // else
      arr.swap(i - reversed_compared_usize, i);
      i -= reversed_compared_usize;
      {  // if i == 0
        let i_being_zero = usize::from(i == 0);
        let reversed_i_being_zero = usize::from(!(i == 0));
        
        i = b * i_being_zero * reversed_compared_usize +
          i * reversed_i_being_zero;
        
        b += 1 * i_being_zero * reversed_compared_usize +
          b * reversed_i_being_zero;
      }
    }
  }

  arr
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn descending() {
    let mut vec = vec![3, 2, 1];
    let gnomed = gnome_sort(&vec);
    for i in 1..gnomed.len() {
      assert!(gnomed[i - 1] <= gnomed[i]);
    }
  }

  #[test]
  fn ascending() {
    let mut vec = vec![-4, 2, 3, 4, 7, 10];
    let gnomed = gnome_sort(&vec);
    for i in 1..gnomed.len() {
      assert!(gnomed[i - 1] <= gnomed[i]);
    }
  }

  #[test]
  fn duplicates() {
    let mut vec = vec![-4, -4, 3, 7, 4, 15];
    let gnomed = gnome_sort(&vec);
    for i in 1..gnomed.len() {
      assert!(gnomed[i - 1] <= gnomed[i]);
    }
  }
}
