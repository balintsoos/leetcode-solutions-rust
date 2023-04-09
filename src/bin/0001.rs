fn main() {}

pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            match map.get(num) {
                Some(&j) => return vec![j, i as i32],
                None => {
                    map.insert(target - num, i as i32);
                }
            }
        }
        unreachable!();
    }
}

#[test]
fn test() {
    assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
}
