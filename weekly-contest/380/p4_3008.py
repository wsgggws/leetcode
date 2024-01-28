from typing import List


class Solution:
    def beautifulIndices(self, s: str, a: str, b: str, k: int) -> List[int]:
        # TODO:
        ...


if __name__ == "__main__":
    assert Solution().beautifulIndices(s="isawsquirrelnearmysquirrelhouseohmy", a="my", b="squirrel", k=15) == [16, 33]
    assert Solution().beautifulIndices(s="abcd", a="a", b="a", k=4) == [0]
