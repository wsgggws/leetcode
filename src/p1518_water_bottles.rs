// 1518. Water BottlesMy SubmissionsBack to Contest
// Difficulty:Easy
// Given numBottles full water bottles, you can exchange numExchange empty water bottles for one full water bottle.

// The operation of drinking a full water bottle turns it into an empty bottle.

// Return the maximum number of water bottles you can drink.

 

// Example 1:



// Input: numBottles = 9, numExchange = 3
// Output: 13
// Explanation: You can exchange 3 empty bottles to get 1 full water bottle.
// Number of water bottles you can drink: 9 + 3 + 1 = 13.
// Example 2:



// Input: numBottles = 15, numExchange = 4
// Output: 19
// Explanation: You can exchange 4 empty bottles to get 1 full water bottle. 
// Number of water bottles you can drink: 15 + 3 + 1 = 19.
// Example 3:

// Input: numBottles = 5, numExchange = 5
// Output: 6
// Example 4:

// Input: numBottles = 2, numExchange = 3
// Output: 2
 

// Constraints:

// 1 <= numBottles <= 100
// 2 <= numExchange <= 100

pub struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut ans = num_bottles;
        let mut empty_number = num_bottles;
        while empty_number >= num_exchange {
            ans += empty_number / num_exchange;
            empty_number = empty_number / num_exchange + empty_number % num_exchange;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn num_water_bottles_exchange_test() {
        assert_eq!(Solution::num_water_bottles(9, 3), 13);
        assert_eq!(Solution::num_water_bottles(15, 4), 19);
        assert_eq!(Solution::num_water_bottles(5, 5), 6);
        assert_eq!(Solution::num_water_bottles(2, 3), 2);
    }
}
