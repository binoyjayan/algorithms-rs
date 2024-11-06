/// Sort the left half and right half of the array
/// Bring the two halves together
/// Complexity: O(n * log(n))
fn merge(a: &mut [i32], aux: &mut [i32], lo: usize, mid: usize, hi: usize) {
    // Copy the elements to the aux array
    aux[lo..=hi].copy_from_slice(&a[lo..=hi]);
    println!("merge: lo:{},mid:{},hi:{}", lo, mid, hi);
    let (mut i, mut j) = (lo, mid + 1);
    for k in lo..=hi {
        if i > mid {
            a[k] = aux[j];
            j += 1;
        } else if j > hi {
            a[k] = aux[i];
            i += 1;
        } else if aux[i] < aux[j] {
            a[k] = aux[i];
            i += 1;
        } else {
            a[k] = aux[j];
            j += 1;
        }
    }
}

fn sort_merge(a: &mut [i32], aux: &mut [i32], lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    println!("sort_merge: lo:{},hi:{}", lo, hi);
    let mid = lo + (hi - lo) / 2;
    sort_merge(a, aux, lo, mid);
    sort_merge(a, aux, mid + 1, hi);
    merge(a, aux, lo, mid, hi);
}

fn merge_sort(a: &mut [i32]) {
    let mut aux = vec![0; a.len()];
    sort_merge(a, &mut aux, 0, a.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![141, 1, 17, -7, -17, -27, 18, 541, 8, 7, 7];
        merge_sort(&mut arr);
        assert_eq!(arr, [-27, -17, -7, 1, 7, 7, 8, 17, 18, 141, 541]);
    }
}
