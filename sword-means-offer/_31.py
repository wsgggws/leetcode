# 剑指 Offer 31. 栈的压入、弹出序列
# 输入两个整数序列，第一个序列表示栈的压入顺序，请判断第二个序列是否为该栈的弹出顺序。
# 假设压入栈的所有数字均不相等。例如，序列 {1,2,3,4,5} 是某栈的压栈序列，序列 {4,5,3,2,1} 是该压栈序列对应的一个弹出序列，但 {4,3,5,1,2} 就不可能是该压栈序列的弹出序列。


# 示例 1：

# 输入：pushed = [1,2,3,4,5], popped = [4,5,3,2,1]
# 输出：true
# 解释：我们可以按以下顺序执行：
# push(1), push(2), push(3), push(4), pop() -> 4,
# push(5), pop() -> 5, pop() -> 3, pop() -> 2, pop() -> 1
# 示例 2：

# 输入：pushed = [1,2,3,4,5], popped = [4,3,5,1,2]
# 输出：false
# 解释：1 不能在 2 之前弹出。


# 提示：

# 0 <= pushed.length == popped.length <= 1000
# 0 <= pushed[i], popped[i] < 1000
# pushed 是 popped 的排列。


from typing import List


class Solution:
    def validateStackSequences(self, pushed: List[int], popped: List[int]) -> bool:
        """
        pushed      [1 2 3 4 5   push_position = 0
        popped      [4 5 3 2 1   pop_position = 0
        stack       []

        case1: stack 为空, stack.append, push_position + 1
        pushed      1 [2 3 4 5   push_position = 1
        popped      [4 5 3 2 1   pop_position = 0
        stack       [] => [1]

        case2: 栈顶 != poped[pop_position], stack.append, push_position + 1
        pushed      1 2 3 [4 5   push_position = 4
        popped      [4 5 3 2 1   pop_position = 0
        stack       [1] => [1, 2, 3, 4]

        case3: 栈顶 == poped[pop_position], stack.pop, pop_position + 1
        pushed      1 2 3 4 [5   push_position = 5
        popped      [4 5 3 2 1   pop_position = 1
        stack       [1, 2, 3, 4] => [1, 2, 3]

        last :
        pushed      1 2 3 4 5[   push_position = 5
        popped      4 [5 3 2 1   pop_position = 1
        stack       [1, 2, 3] => [1, 2, 3, 5]
        因为 push_position 已经走完，
        只需要判断 倒序栈 是否与 popped[pop_position:] 相等？
        """
        stack = []
        push_position, pop_position = 0, 0

        while push_position < len(pushed) and pop_position < len(popped):
            if len(stack) == 0 or stack[-1] != popped[pop_position]:
                stack.append(pushed[push_position])
                push_position += 1
            else:
                stack.pop()
                pop_position += 1
        if stack[::-1] == popped[pop_position:]:
            return True
        return False


if __name__ == "__main__":
    s = Solution()
    assert s.validateStackSequences([], []) is True
    assert s.validateStackSequences([1, 2, 3, 4, 5], [5, 4, 3, 2, 1]) is True
    assert s.validateStackSequences([1, 2, 3, 4, 5], [4, 5, 3, 2, 1]) is True
    assert s.validateStackSequences([1, 2, 3, 4, 5], [4, 3, 5, 1, 2]) is False
