pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // 2以下の場合は、そのまま返却する
        if n <= 2 {
            return n;
        }
        // それ以外の場合は動的計画法で解く
        let mut prev = 1;
        let mut curr = 2;
        let mut tmp: i32;
        // 3からnまでループ
        for _ in 3..=n {
            tmp = curr;
            curr += prev;
            prev = tmp;
        }
        curr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nが2以下の場合にプログラムが動くかどうかを確認する() {
        // nが2以下の場合にプログラムが動くかどうかを確認する
        let want = 2;
        let result = Solution::climb_stairs(2);
        assert_eq!(want, result);
    }

    #[test]
    fn nが3の場合にプログラムが動くかどうかを確認する() {
        let want = 3;
        let result = Solution::climb_stairs(3);
        assert_eq!(want, result);
    }
    #[test]

    fn nが3以上の場合にプログラムが動くかどうかを確認する() {
        let want = 5;
        let result = Solution::climb_stairs(4);
        assert_eq!(want, result);
    }
}
