// 347. Top K Frequent Elements
// Medium

// Given a non-empty array of integers, return the k most frequent elements.

// Example 1:

// Input: nums = [1,1,1,2,2,3], k = 2
// Output: [1,2]
// Example 2:

// Input: nums = [1], k = 1
// Output: [1]
// Note:

// You may assume k is always valid, 1 ≤ k ≤ number of unique elements.
// Your algorithm's time complexity must be better than O(n log n), where n is the array's size.
// It's guaranteed that the answer is unique, in other words the set of the top k frequent elements is unique.
// You can return the answer in any order.

pub struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, usize> = HashMap::new();
        for &num in nums.iter() {
            let counter = hash_map.entry(num).or_insert(0);
            *counter += 1;
        }
        let mut tmp: Vec<(i32, usize)> = hash_map.iter()
            .map(|(&a, &b)| (a, b))
            .collect();

        // 使用nlogn的排序算法, 依赖了Rust语言自带的排序算法，也可以使用堆排序或者归并排序
        tmp.sort_by(|a, b| b.1.cmp(&a.1));

        tmp.iter()
            .map(|&(a, _b)| a)
            .take(k as usize)
            .collect()
    }
}

// #[derive(Debug)]
// struct Heap<T: Ord> {
//     elems: Vec<T>
// }

// impl<T: Ord> Heap<T> {
//     fn new() -> Heap<T> {
//         Heap { elems: Vec::new() }
//     }

//     fn from(elems: Vec<T>) -> Heap<T> {
//         let mut heap = Heap { elems: elems };
//         for i in (0..heap.elems.len()/2).rev() {
//             heap.min_heapify(i)
//         }
//         heap
//     }

//     fn parent(i: usize) -> usize {
//         if i > 0 { (i-1) /2 } else { 0 }
//     }

//     fn left(i: usize) -> usize {
//         i * 2 + 1
//     }
//     fn right(i: usize) -> usize {
//         i * 2 + 2
//     }

//     fn min_heapify(&mut self, i: usize) {
//         let (left, right, mut largest) = (Heap::<T>::left(i), Heap::<T>::right(i), i);
//         if left < self.elems.len() && self.elems[left] < self.elems[largest] {
//             largest = left;
//         }
//         if right < self.elems.len() && self.elems[right] < self.elems[largest] {
//             largest = right;
//         }
//         if largest != i {
//             self.elems.swap(largest, i);
//             self.min_heapify(largest);
//         }
//     }

//     fn push(&mut self, v: T) {
//         self.elems.push(v);
//         let mut i = self.elems.len() - 1;
//         while i > 0 && self.elems[Heap::<T>::parent(i)] < self.elems[i] {
//             self.elems.swap(i, Heap::<T>::parent(i));
//             i = Heap::<T>::parent(i);
//         }
//     }

//     fn pop(&mut self) -> Option<T> {
//         if self.elems.is_empty() {
//             None
//         } else {
//             let b = self.elems.len() - 1;
//             self.elems.swap(0, b);
//             let v = Some(self.elems.pop().unwrap());
//             if !self.elems.is_empty() {
//                 self.min_heapify(0);
//             }
//             v
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn top_k_frequent_test() {
        assert_eq!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2]);
        assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);
    }
}
