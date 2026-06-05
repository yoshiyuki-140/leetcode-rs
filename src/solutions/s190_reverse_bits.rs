pub struct Solution;

impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        // 符号なし32ビットに型変換する
        // 制約が0 <= n <= 2^31-2
        let mut n: u32 = n as u32;

        // 結果を格納する変数を定義する
        let mut result: u32 = 0;
        // 31回、回せばいい
        for _ in 0..31 {
            result |= n & 1;
            // 左シフト
            result <<= 1;
            // 右シフト
            n >>= 1;
        }

        result as i32
    }
}

/* 別解 (https://leetcode.com/problems/reverse-bits/solutions/7582521/rust-with-4bit-lookup-optimization-and-s-g8yx/)
// 4bit optimization means 16 bytes for lookup
// can implement 8bit with 256 lookup
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let lookup:[u8;16] = [0,8,4,12,2,10,6,14,1,9,5,13,3,11,7,15];
        let mut res: [u8; 4] = n.to_le_bytes();

        for b in &mut res {
            let lo = *b & 15;
            let hi = *b >> 4;
            *b = (lookup[lo as usize] << 4) | lookup[hi as usize];
        }
        return i32::from_be_bytes(res);
    }
}
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// 一般的なテスト
    fn test1() {
        let result = Solution::reverse_bits(43261596);
        let want = 964176192;
        assert_eq!(want, result);
    }

    #[test]
    fn test2() {
        let result = Solution::reverse_bits(2147483644);
        let want = 1073741822;
        assert_eq!(want, result);
    }
}
