pub struct Solution;

/* アルゴリズム言語化
1. buyを0で初期化
2. sellを1で初期化
3. prices.len()回数while実行
4. もし、利益が出ていたら(prices[buy] < prices[sell]であれば)、利益を計算する
    1. 利益が現在の最大利益より大きい場合は、最大利益を更新する
5. 利益が出ていなければ、buy = sell
6. 常にsell += 1
7. 最後に最大利益を返却
*/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = 0;
        let mut sell = 1;
        let mut max_profit = 0;
        while sell < prices.len() {
            if prices[buy] < prices[sell] {
                let profit = prices[sell] - prices[buy];
                if profit > max_profit {
                    max_profit = profit;
                }
            } else {
                buy = sell;
            }
            sell += 1;
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let result = Solution::max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(result, 5);
    }
}
