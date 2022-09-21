# https://leetcode.cn/problems/shu-ju-liu-zhong-de-zhong-wei-shu-lcof/
# import heapq

# 两个列表分别存取一半的元素
# A                     B
# [小数值中的最大值] <= [大数值中的最小值]

# 如何在O(1)的时间复杂度取最大最小值
# => A 设置为大顶堆
# => B 设置为小顶堆


# findMedian
# len(A) == len(B) => A(max) + B(min) / 2.0
# len(A) == len(B) + 1 => A(max) / 1.0


# addNum
# len(A) == len(B), 得往 A 添加一个数值, 该数值得小于 B 里的每一个元素
# =>  push to B, pop from B, push to A, 时间复杂度 O(log2N)

# len(A) != len(B), 得往 B 添加一个数值, 该数值得大于 A 里的每一个元素
# =>  push to A, pop from A, push to B, 时间复杂度 O(log2N)

# 如何设置小顶堆
# heapq 提供的是大顶堆，通过对每个数据取负，显然依然是大顶堆，却实现了小顶堆的功能
# or 自己造轮子


# class MedianFinder:
#     def __init__(self):
#         """
#         initialize your data structure here.
#         """
#         self.left_max_heap = []
#         self.right_min_heap = []

#     def addNum(self, num: int) -> None:
#         if len(self.left_max_heap) == len(self.right_min_heap):
#             value = heapq.heappushpop(self.right_min_heap, -num)
#             heapq.heappush(self.left_max_heap, -value)
#         else:
#             value = heapq.heappushpop(self.left_max_heap, num)
#             heapq.heappush(self.right_min_heap, -value)

#     def findMedian(self) -> float:
#         if len(self.left_max_heap) == 0:
#             return
#         elif len(self.left_max_heap) == len(self.right_min_heap):
#             return (self.left_max_heap[0] - self.right_min_heap[0]) / 2.0
#         else:
#             return self.left_max_heap[0] / 1.0


class MedianFinder:
    def __init__(self):
        """
        initialize your data structure here.
        """
        self.nums = []

    def addNum(self, num: int) -> None:
        self.nums.append(num)

    def findMedian(self) -> float:
        count = len(self.nums)
        if count == 0:
            return
        self.nums.sort()
        if count % 2 == 0:
            return (self.nums[count // 2] + self.nums[count // 2 - 1]) / 2.0
        else:
            return self.nums[count // 2] / 1.0


if __name__ == "__main__":
    finder = MedianFinder()
    assert finder.findMedian() is None
    finder.addNum(1)
    finder.addNum(2)
    assert abs(finder.findMedian() - 1.5) <= 10e-8
    finder.addNum(3)
    assert abs(finder.findMedian() - 2.0) <= 10e-8
