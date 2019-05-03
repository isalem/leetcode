use std::collections::hash_map::Entry;
use std::collections::HashMap;

/// # [Two Sum](https://leetcode.com/problems/two-sum/)
///
/// Given an array of integers, return indices of the two numbers such that they add up to a
/// specific target.
///
/// You may assume that each input would have exactly one solution, and you may not use the same
/// element twice.
/// 
/// ## Example
/// 
/// ```
/// let nums = vec![2, 7, 11, 15];
/// let target = 9;
/// let indices = leetcode::easy::two_sum(nums, target);
/// assert_eq!(0, indices[0]);
/// assert_eq!(1, indices[1]);
/// ```
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut complement_map = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        if let Entry::Vacant(v) = complement_map.entry(num) {
            v.insert(index);
        }
    }

    for (first_index, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(second_index) = complement_map.get(&complement) {
            if first_index != *second_index {
                return [first_index as i32, *second_index as i32].to_vec();
            }
        }
    }

    unreachable!("Input doesn't have a solution");
}
