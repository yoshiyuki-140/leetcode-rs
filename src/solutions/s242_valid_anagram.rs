pub struct Solution;

/*
アルゴリズム言語化
ハッシュマップを使って、sが出現したら+1してtが出現したら-1することで、同時にカウントを行い、プログラム
 */

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        // key : 文字列, value : 出現頻度計測数値
        let mut hash_map: std::collections::HashMap<char, i32> =
            std::collections::HashMap::with_capacity(s.len());

        for (cs, ct) in s.chars().zip(t.chars()) {
            // csがあるかどうかを確認してない場合は0を挿入しておく
            // あった場合はその値に+1する
            *hash_map.entry(cs).or_insert(0) += 1;
            // tsがあるかどうかを確認してない場合は0を挿入しておく
            // あった場合はその値に-1する
            *hash_map.entry(ct).or_insert(0) -= 1;
        }

        // 全てがゼロであることを判定する
        /* この判定ロジックは以下のように一行で記述可能
        hash_map.values().all(|&val| val == 0)
         */
        for (_, val) in hash_map {
            // もし0でないものが1つでもあればfalseを返却する
            if val != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::is_anagram(String::from("anagram"), String::from("nagaram"));
        assert_eq!(result, true);
    }
    #[test]
    fn test_example2() {
        let result = Solution::is_anagram(String::from("rat"), String::from("car"));
        assert_eq!(result, false);
    }
}
