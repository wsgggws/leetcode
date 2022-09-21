class CQueue:
    """
      s1     s2
    value3
    value2
    value1  ------

      s1      s2
            value1
            value2
            value3
    """

    def __init__(self):
        self.stack1 = []
        self.stack2 = []

    def appendTail(self, value: int) -> None:
        self.stack1.append(value)

    def deleteHead(self) -> int:
        if len(self.stack2) > 0:
            value = self.stack2.pop()
        elif len(self.stack1) > 0:
            self.stack2 = self.stack1[::-1]
            self.stack1 = []
            value = self.stack2.pop()
        else:
            value = -1
        return value


if __name__ == "__main__":
    obj = CQueue()
    assert obj.deleteHead() == -1
    obj.appendTail(5)
    obj.appendTail(2)
    assert obj.deleteHead() == 5
    assert obj.deleteHead() == 2
