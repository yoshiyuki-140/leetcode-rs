pub struct Solution;

/* アルゴリズム

    同じ数字同士でxorをすると0になるという性質を使用
    配列内の要素を全てxor演算して残った数がsingle number

*/

// for文を使った解法
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut acc: i32 = 0;
        for n in nums {
            acc ^= n;
        }
        acc
    }
}

// Rustらしく最適化した解法
#[cfg(any())]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        // xorの解法
        nums.into_iter().fold(0, |acc, x| acc ^ x)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn test_single_number() {
        let result = Solution::single_number(vec![2, 2, 1]);
        let want = 1;
        assert_eq!(want, result);
    }

    #[test]
    pub fn test2_single_number() {
        let result = Solution::single_number(vec![4, 1, 2, 1, 2]);
        let want = 4;
        assert_eq!(want, result);
    }
}
