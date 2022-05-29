pub fn partition(arr: &mut [isize], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    while i < j {
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        while arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i < j {  // compare and multiply by answer to possible make cancel out
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot);
    i
}

pub fn quick_sort(arr: &mut [isize]) {

}


#[cfg(test)]
mod tests {
    use super::*;
}