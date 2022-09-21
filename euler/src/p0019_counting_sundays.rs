// Counting Sundays
// Problem 19
// You are given the following information, but you may prefer to do some research for yourself.

// 1 Jan 1900 was a Monday.
// Thirty days has September,
// April, June and November.
// All the rest have thirty-one,
// Saving February alone,
// Which has twenty-eight, rain or shine.
// And on leap years, twenty-nine.
// A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

extern crate chrono;

use chrono::prelude::*;
use chrono::Duration;
 
pub struct Solution {}

impl Solution {
    pub fn counting_sundays() -> u32 {
        let mut result = 0;
        let mut start = Utc.ymd(1901, 1, 1);
        while start <= Utc.ymd(2000, 12, 31) {
            if start.weekday() == Weekday::Sun && start.day() == 1 {
                result += 1;
            }
            start = start + Duration::days(1);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_sundays_test() {
        assert_eq!(Solution::counting_sundays(), 171);
    }
}
