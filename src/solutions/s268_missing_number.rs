pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // 和を計算
        let mut sum: i32 = ((nums.len() + 1) * (nums.len()) / 2) as i32;
        for n in nums {
            sum -= n;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_missing_number() {
        let want = Solution::missing_number(vec![3, 0, 1]);
        let result = 2;
        assert_eq!(want, result);
    }
    #[test]
    pub fn test2_missing_number() {
        let want = Solution::missing_number(vec![0, 1]);
        let result = 2;
        assert_eq!(want, result);
    }
}
