def _str2int(nums: str) -> int:
    result = 0
    tent = 0
    for num in nums[::-1]:
        result += int(num) * (10 ** tent)
        tent += 1
    return result


def _check(chars: str) -> bool:
    if chars.count(".") > 1 or chars.count("+") > 1 or chars.count("-") > 1 or len(chars) == 0:
        return False
    for char in chars:
        if (char < "0" or char > "9") and char not in "+-.":
            return False
    return True


def str2float(chars: str) -> float:
    if not _check(chars):
        raise TypeError(f"can not parse {chars} to a float")

    if chars.count(".") == 0:
        nums, digits = chars, ""
    else:
        nums, digits = chars.split(".")

    flags = 1
    if len(nums) == 0:
        nums = ""
    else:
        if nums[0] == "-":
            flags = -1
        if nums[0] in "-+":
            nums = nums[1:]
    return flags * (_str2int(nums) + _str2int(digits) / (10 ** len(digits)))


if __name__ == "__main__":

    assert str2float(".45") == 0.45
    assert str2float("45") == 45
    assert str2float("05") == 5
    assert str2float("123.45") == 123.45
    assert str2float("+123.45") == 123.45
    assert str2float("-123.45") == -123.45
    assert str2float("-103.45") == -103.45
    assert str2float("-103.405") == -103.405
    assert str2float("-103.4005") == -103.4005
    assert str2float("-103.4005000007") == -103.4005000007
    assert str2float("-103.0000000007") == -103.0000000007
