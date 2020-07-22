// 215. Kth Largest Element in an Array
// Medium

// Find the kth largest element in an unsorted array. Note that it is the kth largest element in the sorted order, not the kth distinct element.

// Example 1:

// Input: [3,2,1,5,6,4] and k = 2
// Output: 5
// Example 2:

// Input: [3,2,3,1,2,4,5,5,6] and k = 4
// Output: 4
// Note:
// You may assume k is always valid, 1 ≤ k ≤ array's length.

pub struct Solution {}

impl Solution {
    // 直接使用排序算法
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut array = nums.clone();
        array.sort_by(|a, b| b.cmp(a));
        *array.iter().nth(k as usize - 1).unwrap()
    }

    // 使用堆排序, 得用大顶堆下沉k次
    pub fn find_kth_largest_by_heapsort(nums: Vec<i32>, k: i32) -> i32 {
        let mut array = nums.clone();
        let lens = nums.len();
        for i in (0..lens / 2).rev() {
            Solution::heap_adjust(&mut array, i, lens-1);
        }
        let mut count = 0;
        for i in (1..lens).rev() {
            array.swap(0, i);
            // 到第K次下沉完成后，提前返回并结束
            count += 1;
            if count == k {
                return array[i];
            }
            Solution::heap_adjust(&mut array, 0, i-1);
        }
        // 最后一个元素，在最大堆顶
        array[0]
    }
    fn heap_adjust(array: &mut Vec<i32>, start: usize, end: usize) {
        let mut parent: usize = start;
        let mut son: usize = 2 * start + 1;
        while son <= end {
            if son + 1 <= end && array[son] < array[son + 1] {
                son += 1;
            }
            if array[parent] > array[son] {
                return;
            } else {
                array.swap(parent, son);
                parent = son;
                son = parent * 2 + 1;
            }
        }
    }
    // 使用快速排序
    pub fn find_kth_largest_by_quicksort(nums: Vec<i32>, k: i32) -> i32 {
        let mut array = nums.clone();
        Solution::quicksort(&mut array, 0, nums.len() - 1, nums.len() - k as usize)
    }

    fn quicksort(array: &mut Vec<i32>, left: usize, right: usize, k: usize) -> i32 {
        let pos = Solution::partition(array, left, right);
        if pos == k {
            return array[pos];
        } else if k > pos {
            // 在右边
            return Solution::quicksort(array, pos + 1, right, k);
        } else {
            // 在左边
            return Solution::quicksort(array, left, pos - 1, k);
        }
    }
    fn partition(array: &mut Vec<i32>, left: usize, right: usize) -> usize {
        let (mut left, mut right) = (left, right);
        let key = array[left];
        while left < right {
            while left < right && array[right] >= key {
                right -= 1;
            }
            if left < right {
                array[left] = array[right];
                left += 1;
            }
            while left < right && array[left] <= key {
                left += 1;
            }
            if left < right {
                array[right] = array[left];
                right -= 1;
            }
        }
        array[left] = key;
        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_kth_largest_test() {
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
    }

    #[test]
    fn find_kth_largest_by_heapsort_test() {
        assert_eq!(Solution::find_kth_largest_by_heapsort(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest_by_heapsort(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
        assert_eq!(Solution::find_kth_largest_by_heapsort(vec![1, 2, 3, 4, 5, 6], 6), 1);
        assert_eq!(Solution::find_kth_largest_by_heapsort(vec![6, 5, 4, 3, 2, 1], 6), 1);
        assert_eq!(Solution::find_kth_largest_by_heapsort(vec![1, 2], 1), 2);
        assert_eq!(Solution::find_kth_largest_by_heapsort(vec![1], 1), 1);
    }

    #[test]
    fn find_kth_largest_by_quicksort_test() {
        assert_eq!(Solution::find_kth_largest_by_quicksort(vec![3, 2, 1, 5, 6, 4], 2), 5);
        assert_eq!(Solution::find_kth_largest_by_quicksort(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
        assert_eq!(Solution::find_kth_largest_by_quicksort(vec![1, 2, 3, 4, 5, 6], 6), 1);
        assert_eq!(Solution::find_kth_largest_by_quicksort(vec![6, 5, 4, 3, 2, 1], 6), 1);
        assert_eq!(Solution::find_kth_largest_by_quicksort(vec![1, 2], 1), 2);
        assert_eq!(Solution::find_kth_largest_by_quicksort(vec![1], 1), 1);
    }
}
