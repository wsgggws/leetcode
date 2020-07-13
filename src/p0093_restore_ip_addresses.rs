// 93. Restore IP Addresses
// Medium

// Given a string containing only digits, restore it by returning all possible valid IP address combinations.

// A valid IP address consists of exactly four integers (each integer is between 0 and 255) separated by single points.

// Example:

// Input: "25525511135"
// Output: ["255.255.11.135", "255.255.111.35"]

pub struct Solution {}

impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut ip_addresses: Vec<String> = vec![];
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    if i+j+k < s.len() && Solution::is_ip_address(&s[0..i], &s[i..i+j], &s[i+j..i+j+k], &s[i+j+k..]) {
                        let ip_address = s[0..i].to_owned() 
                            + &"."
                            + &s[i..i+j]
                            + &"."
                            + &s[i+j..i+j+k]
                            + &"."
                            + &s[i+j+k..];
                        ip_addresses.push(ip_address);
                    }
                }
            }
        }
        ip_addresses
    }
    fn is_ip_address(s1: &str, s2: &str, s3: &str, s4: &str) -> bool {
        // 最后一位数字大于3位数绝对不合格
        if s4.len() > 3 {
            return false;
        }
        // 不允许有前导0，i.e: "01"
        if (s1.len() > 1 && &s1[0..1] == "0")
            || (s2.len() > 1 && &s2[0..1] == "0")
            || (s3.len() > 1 && &s3[0..1] == "0") 
            || (s4.len() > 1 && &s4[0..1] == "0") 
        {
            return false;
        } else {
            if s1.parse::<i32>().unwrap() > 255 { return false; }
            if s2.parse::<i32>().unwrap() > 255 { return false; }
            if s3.parse::<i32>().unwrap() > 255 { return false; }
            if s4.parse::<i32>().unwrap() > 255 { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restore_ip_addresses_test() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_owned()),
            vec!["255.255.11.135", "255.255.111.35"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("1111".to_owned()),
            vec!["1.1.1.1"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_owned()),
            vec!["0.0.0.0"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("010010".to_owned()),
            vec!["0.10.0.10", "0.100.1.0"]
        );
        assert_eq!(
            Solution::restore_ip_addresses("255255255255".to_owned()),
            vec!["255.255.255.255"]
        );

        let expected_addresses: Vec<String> = Vec::new();
        assert_eq!(
            Solution::restore_ip_addresses("55555555555555555".to_owned()),
            expected_addresses
        );
    }
}
