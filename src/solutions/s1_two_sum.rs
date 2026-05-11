pub struct Solution; // 空の構造体を定義

/*
    アルゴリズム言語化
    1. ハッシュマップを用意する。格納するのは(key:target-n, nのindex)
    2. forループでnumsを走査して、もしnというキーを持っているならば、vec![hashMap[target-n],i]を返却する
*/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 1. ハッシュマップを用意する
        let mut map: std::collections::HashMap<i32, usize> =
            std::collections::HashMap::with_capacity(nums.len()); // capacityを読み込むとプログラムが安定する

        // 2. hashMapの中にあるかどうかを判定する
        for (i, &n) in nums.iter().enumerate() {
            // hashMapの中にあるかどうかを判定して、存在すればそれを返却する
            if let Some(&prev_index) = map.get(&n) {
                return vec![prev_index as i32, i as i32];
            }
            // 現在の値の相方が必要な値として、今のインデックスを保存
            map.insert(target - n, i);
        }
        // ここには到達しないけど返却
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 同じファイル内のSolutionをインポート
    use std::vec;

    #[test]
    fn test_example1() {
        // 呼び出し例
        let result = Solution::two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }
    #[test]
    fn test_example2() {
        let result = Solution::two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }
}
