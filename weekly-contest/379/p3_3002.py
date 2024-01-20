from collections import Counter
from typing import List


class Solution:
    def maximumSetSize(self, nums1: List[int], nums2: List[int]) -> int:
        counter1, counter2 = Counter(nums1), Counter(nums2)
        # step1: 先选择不重复的数字
        cnt1, cnt2 = 0, 0
        select_numbers = []
        for number in counter1.keys():
            if number not in counter2:
                cnt1 += 1
                select_numbers.append(number)
            if cnt1 >= len(nums1) // 2:
                break
        for number in select_numbers:
            counter1.pop(number)
        select_numbers = []
        for number in counter2.keys():
            if number not in counter1:
                cnt2 += 1
                select_numbers.append(number)
            if cnt2 >= len(nums2) // 2:
                break
        for number in select_numbers:
            counter2.pop(number)

        # step2: 把重复的数字尽量选够
        if cnt1 == len(nums1) // 2:
            all_counter = counter2
        elif cnt2 == len(nums2) // 2:
            all_counter = counter1
        else:
            all_counter = counter1 + counter2
        cnt3 = 0
        for number, _ in all_counter.items():
            if cnt1 + cnt2 + cnt3 >= len(nums1):
                break
            cnt3 += 1
        return cnt1 + cnt2 + cnt3


if __name__ == "__main__":
    assert Solution().maximumSetSize(nums1=[1, 2, 1, 2], nums2=[1, 1, 1, 1]) == 2
    assert Solution().maximumSetSize(nums1=[1, 2, 3, 4, 5, 6], nums2=[2, 3, 2, 3, 2, 3]) == 5
    assert Solution().maximumSetSize(nums1=[1, 1, 2, 2, 3, 3], nums2=[4, 4, 5, 5, 6, 6]) == 6
    assert Solution().maximumSetSize(nums1=[2, 6, 1, 10, 6, 6, 5, 6], nums2=[2, 7, 7, 10, 9, 1, 9, 4]) == 8
    assert (
        Solution().maximumSetSize(nums1=[3, 1, 2, 3, 7, 10, 10, 6, 3, 10], nums2=[9, 7, 1, 9, 5, 3, 2, 4, 5, 5]) == 9
    )
