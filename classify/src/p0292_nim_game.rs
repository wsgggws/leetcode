pub struct Solution {}

impl Solution {
    // 小时候老爸跟我经常玩报数的游戏；）
    pub fn can_win_nim(n: i32) -> bool {
        n % 4 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_win_nim_test() {
        assert_eq!(Solution::can_win_nim(4), false);
        assert_eq!(Solution::can_win_nim(1), true);
        assert_eq!(Solution::can_win_nim(2), true);
        assert_eq!(Solution::can_win_nim(3), true);
        assert_eq!(Solution::can_win_nim(5), true);
        assert_eq!(Solution::can_win_nim(6), true);
        assert_eq!(Solution::can_win_nim(7), true);
    }
}
