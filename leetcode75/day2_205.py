class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        """
        s 没有映射
            t 有被映射过             -> False
            t 没有被映射过           -> 添加映射
        s 有映射
            t 的值与与映射值一致      -> continue
            t 的值与与映射值不一致    -> False
        """
        maps = {}
        for s1, t1 in zip(s, t):
            if s1 not in maps:
                if t1 in maps.values():
                    return False
                else:
                    maps[s1] = t1
            else:
                if t1 != maps[s1]:
                    return False
                else:
                    continue
        return True


if __name__ == "__main__":
    assert Solution().isIsomorphic("egg", "add") is True
    assert Solution().isIsomorphic("foo", "bar") is False
    assert Solution().isIsomorphic("paper", "title") is True
    assert Solution().isIsomorphic("badc", "baba") is False
