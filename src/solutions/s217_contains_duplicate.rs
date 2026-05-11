use std::collections::HashMap;

pub struct Solution;

/*
アルゴリズム言語化
ハッシュマップを使って、今まで出てきたやつを判定する
時間計算量はO(n)
 */

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // ハッシュマップを使えばいい気がします
        let mut hash_map: HashMap<i32, bool> = HashMap::with_capacity(nums.len());
        // ハッシュマップを使うのがいい気がします。

        for n in nums {
            // 値がハッシュマップに存在するかどうかを確認する
            if hash_map.contains_key(&n) {
                return true;
            }
            hash_map.insert(n, true);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(result, true);
    }
}
