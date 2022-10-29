from typing import List


class Solution:
    def asteroidCollision(self, asteroids: List[int]) -> List[int]:
        """
        利用栈来模拟
        1. 空栈: 入栈
        2. 栈顶方向左: 入栈
        3. 栈顶方向右:
            a. 相同方向: 入栈
            b. 相反方向:
                a. 栈顶 > 新行星: 跳过该行星
                b. 栈顶 = 新行星: 出栈
                c. 栈顶向右 < 新行星: 出栈, loop 3.c.
                    I.没有栈顶了: 入栈该行星
                    II. 栈顶向左: 入栈
                    III. 相等: 出栈
                    IIII. 栈顶>新行星: 跳过该行星
        """
        if len(asteroids) < 2:
            raise KeyError
        stack = []
        for weight in asteroids:
            if not stack or stack[-1] < 0:
                stack.append(weight)
            else:
                if stack[-1] * weight > 0:
                    stack.append(weight)
                else:
                    if stack[-1] > abs(weight):
                        continue
                    elif stack[-1] == abs(weight):
                        stack = stack[:-1]
                    else:
                        while stack and stack[-1] > 0 and stack[-1] < abs(weight):
                            stack = stack[:-1]
                        if not stack or stack[-1] < 0:
                            stack.append(weight)
                        elif stack[-1] == abs(weight):
                            stack = stack[:-1]
                        elif stack[-1] > abs(weight):
                            continue
        return stack


if __name__ == "__main__":
    assert Solution().asteroidCollision(asteroids=[5, 10, -5]) == [5, 10]
    assert Solution().asteroidCollision(asteroids=[5, -5]) == []
    assert Solution().asteroidCollision(asteroids=[10, 2, -5]) == [10]
    assert Solution().asteroidCollision(asteroids=[2, 10, 2, -5]) == [2, 10]
    assert Solution().asteroidCollision(asteroids=[2, -2, 10, 2, -5]) == [10]
    assert Solution().asteroidCollision(asteroids=[-2, -1, 1, 2]) == [-2, -1, 1, 2]
    assert Solution().asteroidCollision(asteroids=[-2, -2, 1, -2]) == [-2, -2, -2]
    assert Solution().asteroidCollision(asteroids=[-2, -1, 1, -2]) == [-2, -1, -2]
