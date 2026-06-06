pub struct Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        // ループしてカウントするのが効率的か？
        let mut n = n;
        let mut count = 0;
        let mut bit;
        while n > 0 {
            // 下位1ビットを取り出す
            bit = n & 1;
            count += bit;
            // 1bit右シフト
            n >>= 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weight() {
        let want = 3;
        let result = Solution::hamming_weight(11);
        assert_eq!(want, result);
    }
}
