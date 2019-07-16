use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_indexs = HashMap::new();
        let mut key = 0;
        for value in nums.iter() {
            let nums_index = target - value;
            if nums_indexs.contains_key(&nums_index) {
                return vec![nums_indexs[&nums_index], key];
             }
            nums_indexs.insert(value, key);
            key += 1;
        }
        return vec![];
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }
}