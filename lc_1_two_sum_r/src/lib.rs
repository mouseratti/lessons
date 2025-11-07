#![crate_type = "lib"]

///
/// ```
/// assert_eq!(lc_1_two_sum_r::find_sum_components(vec![3,3], 6), vec![0,1]);
/// ```
/// ```
/// assert_eq!(lc_1_two_sum_r::find_sum_components(vec![1,4,5], 9), vec![1,2]);
/// ```

pub fn find_sum_components(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut storage: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        let addend2 = target - nums[i];
        match storage.get(&addend2) {
            Some(addend2_idx) => return vec![*addend2_idx, i as i32],
            None => storage.insert(nums[i], i as i32),
        };
    }
    return vec![0, 0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge() {
        assert_eq!(find_sum_components(vec![1, 4, 5], 9), vec![1, 2]);
    }
}
