def _trans(elements: list) -> str:
    if not elements:
        return ""
    if all(ele.startswith("零") for ele in elements):
        return elements[0][0]
    # 从元开始都是零的情况
    tag = 0
    if elements[0].startswith("零"):
        tag = 0
        for element in elements:
            if element.startswith("零"):
                tag += 1
            else:
                break
    suffix = ""
    if tag > 0:
        suffix = elements[0][1]

    elements = elements[tag:]

    # 处理中间是零的情况，特别要注意连着零
    result = [elements[0]]
    length = len(elements)
    for index in range(1, length):
        if not elements[index].startswith("零"):
            result.append(elements[index])
        else:
            if result[-1][0] == "零":
                continue
            else:
                result.append("零")
    return f"{''.join(reversed(result))}{suffix}"


def money2hans(money: float) -> str:
    if money == 0:
        return "零元整"
    maps = {
        0: "零",
        1: "壹",
        2: "贰",
        3: "叁",
        4: "肆",
        5: "伍",
        6: "陆",
        7: "柒",
        8: "捌",
        9: "玖",
    }
    units = ["分", "角", "元", "拾", "佰", "仟", "万", "拾", "佰", "仟", "亿"]

    # 四舍五入
    if (money * 1000 % 10) >= 5.0:
        money = int(money * 100) + 1
    else:
        money = int(money * 100)

    result = []
    index = 0
    while money > 0:
        result.append(f"{maps[money % 10]}{units[index]}")
        money //= 10
        index += 1

    # 处理角分
    feng = ""
    if len(result) == 1:
        feng = result[0]
    elif result[0].startswith("零") and result[1].startswith("零"):
        feng = "整"
    elif result[0].startswith("零") and not result[1].startswith("零"):
        feng = f"{result[1]}"
    elif not result[0].startswith("零") and result[1].startswith("零"):
        feng = f"零{result[0]}"
    else:
        feng = f"{result[1]}{result[0]}"

    result = result[2:]
    # 只有角分的支持结束
    if not result:
        return feng

    yuan = _trans(result[:4])
    result = result[4:]
    wang = _trans(result[:4])
    result = result[4:]
    yi = _trans(result[:1])
    if len(yi) == 1:
        yi = ""
    if len(wang) == 1:
        wang = ""
    if len(yuan) == 1:
        yuan = "元"

    # 注意壹拾元整
    hans = f"{yi}{wang}{yuan}{feng}"
    if hans.startswith("壹拾"):
        hans = hans[1:]
    return hans


if __name__ == "__main__":
    assert money2hans(10234.567) == "壹万零贰佰叁拾肆元伍角柒分"
    assert money2hans(123045607.12) == "壹亿贰仟叁佰零肆万伍仟陆佰零柒元壹角贰分"
    assert money2hans(98) == "玖拾捌元整"
    assert money2hans(98.00) == "玖拾捌元整"
    assert money2hans(580) == "伍佰捌拾元整"
    assert money2hans(7800) == "柒仟捌佰元整"
    assert money2hans(0.5) == "伍角"
    assert money2hans(0.05) == "伍分"
    assert money2hans(0) == "零元整"
    assert money2hans(0.0) == "零元整"
    assert money2hans(1) == "壹元整"
    assert money2hans(10) == "拾元整"
    assert money2hans(20) == "贰拾元整"
    assert money2hans(700800) == "柒拾万零捌佰元整"
    assert money2hans(100000001) == "壹亿零壹元整"
    assert money2hans(100000001.255) == "壹亿零壹元贰角陆分"
    assert money2hans(103000001.255) == "壹亿零叁佰万零壹元贰角陆分"
    assert money2hans(103000201.255) == "壹亿零叁佰万零贰佰零壹元贰角陆分"
