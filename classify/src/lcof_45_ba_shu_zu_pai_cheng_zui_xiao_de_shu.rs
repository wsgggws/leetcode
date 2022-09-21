pub struct Solution {}

use std::cmp::Ordering;
impl Solution {
    pub fn min_number(nums: Vec<i32>) -> String {
        let mut result: Vec<(usize, String)> = vec![];
        for (index, &num) in nums.iter().enumerate() {
            result.push((index, num.to_string()));
        }
        result.sort_by(|a, b| Solution::cmp(a, b));

        // println!("result: {:?}", result);
        let mut ans: String = String::new();
        for &(index, _) in result.iter() {
            ans.push_str(&nums[index].to_string());
        }
        ans
    }
    fn cmp(a: &(usize, String), b: &(usize, String)) -> Ordering {
        let str_a = a.1.clone() + &b.1.clone();
        let str_b = b.1.clone() + &a.1.clone();
        if str_a < str_b {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

// TODO, 这种排序方式拼接过不了
// use std::cmp::Ordering;
// impl Solution {
//     pub fn min_number(nums: Vec<i32>) -> String {
//         let max_bit = nums.iter().max().unwrap().to_string().len();
//         let mut result: Vec<(usize, String, String)> = vec![];
//         for (index, &num) in nums.iter().enumerate() {
//             let first_digit: &str = &num.to_string()[0..1];
//             let mut digit_str: String = num.to_string();
//             let mut lens = digit_str.len();
//             while lens < max_bit {
//                 digit_str.push_str(first_digit);
//                 lens += 1;
//             }
//             result.push((index, digit_str, num.to_string()));
//         }
//         result.sort_by(|a, b| Solution::cmp(a, b));

//         println!("result: {:?}", result);
//         let mut ans: String = String::new();
//         for &(index, _, _) in result.iter() {
//             ans.push_str(&nums[index].to_string());
//         }
//         ans
//     }
//     fn cmp(a: &(usize, String, String), b: &(usize, String, String)) -> Ordering {
//         if a.1 < b.1 {
//             return Ordering::Less;
//         } else if a.1 == b.1 && a.2.len() > b.2.len() {
//             return Ordering::Less;
//         } else {
//             return Ordering::Greater;
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_number_test() {
        assert_eq!(Solution::min_number(vec![10, 2]), "102".to_owned());
        assert_eq!(
            Solution::min_number(vec![3, 30, 34, 5, 9]),
            "3033459".to_owned()
        );
        assert_eq!(Solution::min_number(vec![10, 2, 0, 0]), "00102".to_owned());
        assert_eq!(
            Solution::min_number(vec![100, 2, 12, 21]),
            "10012212".to_owned()
        );
        assert_eq!(
            Solution::min_number(vec![824, 938, 1399, 5607, 6973, 5703, 9609, 4398, 8247]),
            "1399439856075703697382478249389609".to_owned()
        );
        assert_eq!(Solution::min_number(vec![12, 121]), "12112".to_owned());

        assert_eq!(
            Solution::min_number(vec![9051,5526,2264,5041,1630,5906,6787,8382,4662,4532,6804,4710,4542,2116,7219,8701,8308,957,8775,4822,396,8995,8597,2304,8902,830,8591,5828,9642,7100,3976,5565,5490,1613,5731,8052,8985,2623,6325,3723,5224,8274,4787,6310,3393,78,3288,7584,7440,5752,351,4555,7265,9959,3866,9854,2709,5817,7272,43,1014,7527,3946,4289,1272,5213,710,1603,2436,8823,5228,2581,771,3700,2109,5638,3402,3910,871,5441,6861,9556,1089,4088,2788,9632,6822,6145,5137,236,683,2869,9525,8161,8374,2439,6028,7813,6406,7519]), "101410891272160316131630210921162264230423624362439258126232709278828693288339334023513700372338663910394639639764088428943453245424555466247104787482250415137521352245228544154905526556556385731575258175828590660286145631063256406678768046822683686171007107219726572727440751975277584771781378805281618274830830883748382859185978701871877588238902898589959051952595569579632964298549959".to_owned());
    }
}
