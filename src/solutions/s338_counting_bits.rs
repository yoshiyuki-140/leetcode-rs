pub struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        // bitをカウントする
        let mut result: Vec<i32> = vec![];
        let mut counter;
        for i in 0..=n {
            let mut idx = i;
            counter = 0;
            while idx > 0 {
                counter += idx & 1;
                idx >>= 1;
            }
            result.push(counter);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_count_bits() {
        let result = Solution::count_bits(2);
        let want = vec![0, 1, 1];
        assert_eq!(want, result);
    }

    #[test]
    pub fn test2_count_bits() {
        let result = Solution::count_bits(5);
        let want = vec![0, 1, 1, 2, 1, 2];
        assert_eq!(want, result);
    }
}
