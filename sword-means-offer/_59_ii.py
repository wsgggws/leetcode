class MaxQueue:
    def __init__(self):
        self.queue = []

    def max_value(self) -> int:
        if len(self.queue) == 0:
            return -1

    def push_back(self, value: int) -> None:
        ...

    def pop_front(self) -> int:
        if len(self.queue) == 0:
            return -1


if __name__ == "__main__":
    s = MaxQueue()
    assert s.pop_front() == -1
    assert s.max_value() == -1
    s.push_back(2)
    s.push_back(1)
    assert s.max_value() == 2
    assert s.pop_front() == 2
    assert s.max_value() == 1

# Your MaxQueue object will be instantiated and called as such:
# obj = MaxQueue()
# param_1 = obj.max_value()
# obj.push_back(value)
# param_3 = obj.pop_front()
