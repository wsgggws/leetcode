class Solution:
    def multiply(self, num1: str, num2: str) -> str:
        list1, list2 = list(int(num) for num in num1), list(int(num) for num in num2)
        ans = 0
        for index1, n1 in enumerate(list1[::-1]):
            for index2, n2 in enumerate(list2[::-1]):
                ans += 10**index1 * n1 * 10**index2 * n2
        return str(ans)


if __name__ == "__main__":
    assert Solution().multiply("2", "3") == "6"
    assert Solution().multiply("0", "3") == "0"
    assert Solution().multiply("123", "456") == "56088"
