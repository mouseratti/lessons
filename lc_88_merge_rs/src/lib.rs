#![crate_type = "lib"]

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut k = nums1.len();
    let mut i = if m > 0 { m as usize - 1 } else { 0 };
    let mut j = if n > 0 { n as usize - 1 } else { 0 };
    let mut nums1_end = if m == 0 { true } else { false };
    let mut nums2_end = if n == 0 { true } else { false };

    loop {
        k -= 1;
        if nums2_end || (!nums1_end && nums1[i] > nums2[j]) {
            nums1[k] = nums1[i];
            if i == 0 {
                nums1_end = true;
            } else {
                i -= 1;
            }
        } else {
            nums1[k] = nums2[j];
            if j == 0 {
                nums2_end = true;
            } else {
                j -= 1;
            }
        }
        if k == 0 {
            break;
        }
    }
}

///
/// ```
/// let mut a = 5;
/// let mut b = 6;
/// lc_88_merge::swapp(&mut a, &mut b);
/// assert_eq!(a, 6);
/// assert_eq!(b, 5);
/// ```
pub fn swapp(i: &mut i32, j: &mut i32) {
    *i = *i ^ *j;
    *j = *i ^ *j;
    *i = *i ^ *j;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        let mut nums1: Vec<i32> = vec![2, 7, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![1, 5, 9];
        let (m, n) = (2, 3);
        merge(&mut nums1, m, &mut nums2, n);
        println!("nums1: {:?}", nums1);
        assert_eq!(nums1, vec![1, 2, 5, 7, 9])
    }

    #[test]
    fn test_merge_m1() {
        let (m, n) = (1, 5);
        let mut nums1: Vec<i32> = vec![4, 0, 0, 0, 0, 0];
        let mut nums2: Vec<i32> = vec![1, 2, 3, 5, 6];
        merge(&mut nums1, m, &mut nums2, n);
        println!("nums1: {:?}", nums1);
        assert_eq!(nums1, vec![1, 2, 3, 4, 5, 6])
    }

    #[test]
    fn test_merge_m0() {
        let mut nums1: Vec<i32> = vec![0];
        let mut nums2: Vec<i32> = vec![1];
        let m = 0;
        let n = 1;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1])
    }

    #[test]
    fn test_merge_n0() {
        let mut nums1: Vec<i32> = vec![1, 2, 3];
        let mut nums2: Vec<i32> = vec![];
        let (m, n) = (3, 0);
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 3])
    }
}
