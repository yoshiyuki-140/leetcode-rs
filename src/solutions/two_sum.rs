pub struct Solution; // 空の構造体を定義

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // ここにアルゴリズムを書く
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec; // 同じファイル内の Solution をインポート

    #[test]
    fn test_example() {
        // 呼び出し例
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
}
