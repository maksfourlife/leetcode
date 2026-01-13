// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii

fn max_profit(prices: &[i32]) -> i32 {
    let mut s = 0;
    for i in 1..prices.len() {
        s += std::cmp::max(0, prices[i] - prices[i - 1]);
    }
    s
}

fn main() {
    assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(&[1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
}
