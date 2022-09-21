from helper import ListNode, create_link, trave_link


class Solution:
    def getKthFromEnd(self, head: ListNode, k: int) -> ListNode:
        if k <= 0:
            return None
        fast = head
        slow = head
        while k > 0 and fast is not None:
            fast = fast.next
            k -= 1

        if fast is None and k > 0:
            raise KeyError

        while fast is not None:
            fast = fast.next
            slow = slow.next

        return slow


if __name__ == "__main__":
    s = Solution()
    nums = [1, 2, 3, 4, 5]
    head = create_link(nums)
    assert trave_link(head) == [1, 2, 3, 4, 5]
    node = s.getKthFromEnd(head, 2)
    assert trave_link(node) == [4, 5]

    nums = [1, 2, 3, 4, 5, 6]
    head = create_link(nums)
    node = s.getKthFromEnd(head, 1)
    assert trave_link(node) == [6]
    node = s.getKthFromEnd(head, 6)
    assert trave_link(node) == [1, 2, 3, 4, 5, 6]
    try:
        node = s.getKthFromEnd(head, 7)
        trave_link(node) == [1, 2, 3, 4, 5, 6]
    except Exception as e:
        assert type(e) == KeyError
