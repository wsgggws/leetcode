pub struct Solution {}


impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut input: i64 = x as i64;
        let mut result: i64 = 0;
        while input != 0 {
            let digit = input % 10;
            result = result * 10 + digit;
            input = input / 10;
        }
        let max_value = 2_i64.pow(31) - 1;
        let min_value = - 2_i64.pow(31);
        if result > max_value || result < min_value{
            return 0;
        }
        result as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(-12300), -321);
        assert_eq!(Solution::reverse((2_i64.pow(31)-1) as i32), 0);
        assert_eq!(Solution::reverse((-2_i64.pow(31)) as i32), 0);
    }
}
