from typing import List, Tuple


class Solution:
    def minimumOperationsToMakeEqual(self, x: int, y: int) -> int:
        queue: List[Tuple[int, int]] = [(x, 0)]
        uniques = set()
        while queue:
            cur_value, cur_cnt = queue[0]
            values: List[int] = [cur_value]
            for i in range(1, len(queue)):
                value, cnt = queue[i]
                if cnt == cur_cnt:
                    values.append(value)
                else:
                    break
            for value in values:
                if value == y:
                    return cur_cnt
                else:
                    if value > y:
                        if value - 1 not in uniques:
                            queue.append((value - 1, cur_cnt + 1))
                            uniques.add(value - 1)
                        if value % 11 == 0 and value // 11 not in uniques:
                            queue.append((value // 11, cur_cnt + 1))
                            uniques.add(value // 11)
                        if value % 5 == 0 and value // 5 not in uniques:
                            queue.append((value // 5, cur_cnt + 1))
                            uniques.add(value // 5)
                    if value + 1 not in uniques:
                        queue.append((value + 1, cur_cnt + 1))
                        uniques.add(value + 1)
            queue = queue[len(values) :]
        return -1


if __name__ == "__main__":
    assert Solution().minimumOperationsToMakeEqual(26, 1) == 3
    assert Solution().minimumOperationsToMakeEqual(54, 2) == 4
    assert Solution().minimumOperationsToMakeEqual(25, 30) == 5
    assert Solution().minimumOperationsToMakeEqual(1, 16) == 15
    assert Solution().minimumOperationsToMakeEqual(89, 57) == 32
