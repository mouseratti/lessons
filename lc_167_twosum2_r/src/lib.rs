///
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(0,5), 2);
/// ```
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(0,3), 1);
/// ```
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(0,2), 1);
/// ```
///
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(0,1), 0);
/// ```
///
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(0,0), 0);
/// ```
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(16,16), 16);
/// ```
///
/// ```
/// assert_eq!(lc_167_twosum2_r::get_mid(1,1), 1);
/// ```
#[inline]
pub fn get_mid(fitst: usize, last: usize) -> usize {
    return (last - fitst) / 2 + fitst;
}

fn bsearch(v: &Vec<i32>, num: i32, start: usize) -> i32 {
    let mut first_idx: usize = start;
    let mut last_idx = v.len() - 1;
    loop {
        if last_idx < first_idx {
            return -1;
        }
        let mid: usize = crate::get_mid(first_idx, last_idx);

        if v[mid] == num {
            return mid as i32;
        }

        // element not found in vector
        if last_idx == first_idx {
            return -1;
        }

        if v[mid] > num {
            last_idx = mid - 1;
        } else {
            first_idx = mid + 1;
        }
    }
}

pub fn sum_of_2(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..numbers.len() {
        let addend2 = target - numbers[i];
        let found = crate::bsearch(&numbers, addend2, i + 1);
        if found != -1 {
            return vec![(i + 1) as i32, found + 1];
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {

    #[test]
    fn sum_of_2_ok() {
        assert_eq!(crate::sum_of_2(vec![-1, 1, 4, 6, 8, 11, 12], 9), vec![2, 5]);
    }

    #[test]
    fn sum_of_2_duplicates() {
        assert_eq!(
            crate::sum_of_2(vec![1, 2, 3, 4, 4, 9, 56, 90], 8),
            vec![4, 5]
        );
    }

    #[test]
    fn bsearch_multiple_elems() {
        let v = vec![-1, 1, 3, 6, 9];
        assert_eq!(crate::bsearch(&v, 3, 0), 2);
    }

    #[test]
    fn bsearch_single_elem() {
        let v = vec![5];
        assert_eq!(crate::bsearch(&v, 5, 0), 0);
    }

    #[test]
    fn bsearch_not_found() {
        assert_eq!(crate::bsearch(&vec![-1, 1, 3, 6, 9], 4, 0), -1);
        assert_eq!(crate::bsearch(&vec![5], 4, 0), -1);
        assert_eq!(crate::bsearch(&vec![2, 5], 4, 0), -1);
    }
}
