def valid(chars: str) -> bool:
    if len(chars) < 2:
        return False
    maps = {
        ")": "(",
        "]": "[",
        "}": "{",
    }
    stack = []
    for char in chars:
        if char in "([{":
            stack.append(char)
        elif char in ")]}":
            if len(stack) == 0:
                return False
            top = stack[-1]
            if top != maps.get(char):
                return False
            stack.pop()
        else:
            return False

    return len(stack) == 0


if __name__ == "__main__":
    assert valid("()[]{}") is True
    assert valid("((({{[]}})))[]{}") is True
    assert valid("((({{[]}})))[]{}(") is False
    assert valid("()(]{}") is False
    assert valid(")()[]{}") is False
    assert valid(")") is False
    assert valid("([a)") is False
