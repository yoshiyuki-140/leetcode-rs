pub struct Solution;

#[cfg(any())]
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

// Rustらしいイテレータを使った書き方
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let l = nums.len();
        // 0からnまでの合計値
        let expected_sum = (l * (l + 1) / 2) as i32;
        // 実際の合計値
        let actual_sum: i32 = nums.iter().sum();
        expected_sum - actual_sum
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
