pub struct Solution {}

impl Solution {
    // 1. 排序后查找 O(nlogn) O(1)
    // 2. 利用HashSet O(n) O(n)

    // O(n) O(1)
    pub fn find_repeat_number(nums: Vec<i32>) -> i32 {
        let mut array = nums.clone();
        for i in 0..nums.len() {
            while array[i] != i as i32 {
                let j = array[i] as usize;
                if array[i] == array[j] {
                    return array[i];
                }
                array.swap(i, j);
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_repeat_number_test() {
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0, 2, 5, 3]), 2);
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 1, 0]), -1);
        assert_eq!(Solution::find_repeat_number(vec![2, 3, 3, 0]), 3);
    }
}
