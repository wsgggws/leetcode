from typing import Set


class Solution:
    def isHappy(self, n: int) -> bool:
        nums: Set[int] = set([n])
        while n != 1:
            n = sum(int(char) * int(char) for char in str(n))
            if n in nums:
                return False
            else:
                nums.add(n)
        return True


if __name__ == "__main__":
    assert Solution().isHappy(19) is True
    assert Solution().isHappy(1) is True
    assert Solution().isHappy(2) is False
