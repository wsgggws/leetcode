from typing import List


class Solution:
    def getLeastNumbers(self, arr: List[int], k: int) -> List[int]:
        max_value = 10000
        counter = [0 for _ in range(max_value + 1)]
        for num in arr:
            counter[num] += 1
        ans = []
        for index in range(max_value + 1):
            if counter[index] > 0:
                while counter[index] > 0 and k > 0:
                    ans.append(index)
                    counter[index] -= 1
                    k -= 1
            if k <= 0:
                break
        return ans


if __name__ == "__main__":
    s = Solution()
    assert s.getLeastNumbers([3, 1, 2], 2) == [1, 2]
    assert s.getLeastNumbers([0, 1, 2, 1], 1) == [0]
    assert s.getLeastNumbers([0, 1, 2, 1], 0) == []
