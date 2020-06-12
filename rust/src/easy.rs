/// # [Two Sum](https://leetcode.com/problems/two-sum/)
///
/// Given an array of integers, return indices of the two numbers such that they add up to
/// a specific target.
///
/// You may assume that each input would have exactly one solution, andyou may not use the same
/// element twice.
mod two_sum {
    struct Solution;
    use std::collections::HashMap;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut complement_map = HashMap::new();

            for (index, num) in nums.iter().enumerate() {
                let complement = target - num;
                match complement_map.get(&complement) {
                    Some(first_index) => return [*first_index as i32, index as i32].to_vec(),
                    None => {
                        complement_map.insert(num, index);
                    }
                }
            }

            unreachable!("Input doesn't have a solution");
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test() {
            let nums = vec![2, 7, 11, 15];
            let target = 9;
            let indices = Solution::two_sum(nums, target);
            assert_eq!(0, indices[0]);
            assert_eq!(1, indices[1]);
        }
    }
}
