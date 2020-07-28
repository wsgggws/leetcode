// 551. Student Attendance Record I
// Easy

// You are given a string representing an attendance record for a student. The record only contains the following three characters:
// 'A' : Absent.
// 'L' : Late.
// 'P' : Present.
// A student could be rewarded if his attendance record doesn't contain more than one 'A' (absent) or more than two continuous 'L' (late).

// You need to return whether the student could be rewarded according to his attendance record.

// Example 1:
// Input: "PPALLP"
// Output: True
// Example 2:
// Input: "PPALLL"
// Output: False

pub struct Solution {}

impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut count_a = 0;
        let chs: Vec<char> = s.chars().collect();
        for i in 0..chs.len() {
            if chs[i] == 'A' {
                count_a += 1;
                if count_a >= 2 {
                    return false
                }
            } else if i + 2 < chs.len() && chs[i] == 'L' && chs[i+1] == 'L' && chs[i+2] == 'L' {
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
    fn check_record_test() {
        assert_eq!(Solution::check_record("PPALLP".to_owned()), true);
        assert_eq!(Solution::check_record("PPALLL".to_owned()), false);
    }
}
