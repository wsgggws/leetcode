// 860. Lemonade Change
// Easy

// At a lemonade stand, each lemonade costs $5.

// Customers are standing in a queue to buy from you, and order one at a time (in the order specified by bills).

// Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill.  You must provide the correct change to each customer, so that the net transaction is that the customer pays $5.

// Note that you don't have any change in hand at first.

// Return true if and only if you can provide every customer with correct change.



// Example 1:

// Input: [5,5,5,10,20]
// Output: true
// Explanation:
// From the first 3 customers, we collect three $5 bills in order.
// From the fourth customer, we collect a $10 bill and give back a $5.
// From the fifth customer, we give a $10 bill and a $5 bill.
// Since all customers got correct change, we output true.
// Example 2:

// Input: [5,5,10]
// Output: true
// Example 3:

// Input: [10,10]
// Output: false
// Example 4:

// Input: [5,5,10,10,20]
// Output: false
// Explanation:
// From the first two customers in order, we collect two $5 bills.
// For the next two customers in order, we collect a $10 bill and give back a $5 bill.
// For the last customer, we can't give change of $15 back because we only have two $10 bills.
// Since not every customer received correct change, the answer is false.


// Note:

// 0 <= bills.length <= 10000
// bills[i] will be either 5, 10, or 20.

pub struct Solution {}

impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut dp5: Vec<i32> = vec![];
        let mut dp10: Vec<i32> = vec![];
        for &bill in bills.iter() {
            match bill {
                5 => dp5.push(5),
                10 => {
                    if dp5.is_empty() {
                        return false
                    } else {
                        dp5.pop();
                        dp10.push(10);
                    }
                },
                20 => {
                    if dp10.len() > 0 && dp5.len() > 0 {
                        dp10.pop();
                        dp5.pop();
                    } else if dp5.len() >= 3 {
                        dp5.pop();
                        dp5.pop();
                        dp5.pop();
                    } else {
                        return false;
                    }
                },
                _ => {}
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lemonade_change_test() {
        assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10]), true);
        assert_eq!(Solution::lemonade_change(vec![10, 10]), false);
        assert_eq!(Solution::lemonade_change(vec![10, 10]), false);
        assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
    }
}
