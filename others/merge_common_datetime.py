from typing import List


class Solution:
    def merge_common_datetime(self, given: List[tuple[int, int]]) -> List[tuple[int, int]]:
        dts = list(sorted(given))
        start, cur = 0, 0
        results = []
        while cur + 1 < len(dts):
            if dts[cur + 1][0] > dts[cur][1]:
                results.append((dts[start][0], dts[cur][1]))
                start = cur + 1
            cur += 1
        results.append((dts[start][0], dts[cur][1]))
        return results


if __name__ == "__main__":
    s = Solution()

    given = [
        (202209101010, 202209101200),
        (202209101010, 202209101020),
        (202209101110, 202209101400),
        (202209101500, 202209101800),
    ]
    expect = [
        (202209101010, 202209101400),
        (202209101500, 202209101800),
    ]
    assert expect == s.merge_common_datetime(given)

    given = [
        (202209101010, 202209101200),
        (202209101010, 202209101020),
        (202209101110, 202209101400),
        (202209101300, 202209101400),
    ]
    expect = [
        (202209101010, 202209101400),
    ]
    assert expect == s.merge_common_datetime(given)

    given = [
        (202209101010, 202209101200),
        (202209101300, 202209101400),
    ]
    expect = [
        (202209101010, 202209101200),
        (202209101300, 202209101400),
    ]
    assert expect == s.merge_common_datetime(given)
