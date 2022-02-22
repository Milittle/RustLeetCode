use std::collections::HashMap;

pub struct Solution{}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut h : HashMap<i32, i32> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let num_left = target - num;
            if h.contains_key(&num_left) {
                return vec![*h.get(&num_left).unwrap(), i as i32];
            }
            h.insert(*num, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let v = vec![2, 7, 11, 5];
        let target = 9;
        let res = Solution::two_sum(v, target);
        assert_eq!(vec![0, 1], res)
    }

    #[test]
    fn test_02() {
        let v = vec![3, 2, 4];
        let target = 6;
        let res = Solution::two_sum(v, target);
        assert_eq!(vec![1, 2], res)
    }

    #[test]
    fn test_03() {
        let v = vec![3, 3];
        let target = 6;
        let res = Solution::two_sum(v, target);
        assert_eq!(vec![0, 1], res)
    }
}