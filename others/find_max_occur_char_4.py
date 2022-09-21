from collections import Counter


def get_max_occur_char(strings: str) -> str:
    letter = ""
    max_count = 0
    counter = Counter()
    for char in strings:
        counter[char] += 1
        if counter[char] > max_count:
            max_count = counter[char]
            letter = char

    return letter


if __name__ == "__main__":
    assert get_max_occur_char("a") == "a"
    assert get_max_occur_char("abcd") == "a"
    assert get_max_occur_char("abbc") == "b"
    assert get_max_occur_char("abcbcdc") == "c"
    assert get_max_occur_char("aaaaa") == "a"
    assert get_max_occur_char("") == ""
