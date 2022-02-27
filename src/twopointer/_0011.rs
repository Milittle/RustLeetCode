pub struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut res;
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        res = i32::min(height[left], height[right]) * (right - left) as i32;

        while left < right {
            if height[left] <= height[right] {
                left += 1;
            } else {
                right -= 1;
            }
            let tmp_res = i32::min(height[left], height[right]) * (right - left) as i32;
            res = i32::max(res, tmp_res);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_01() {
        let input = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let res = Solution::max_area(input);
        assert_eq!(res, 49);
    }

    #[test]
    fn test_02() {
        let input = vec![1, 1];
        let res = Solution::max_area(input);
        assert_eq!(res, 1);
    }
}
