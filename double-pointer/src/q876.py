# Definition for singly-linked list.
import pytest

class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def middleNode(self, head: ListNode) -> ListNode:
        if head.next is None:
            return head

        fp = head
        np = head

        while True:
            np = np.next
            if np is None:
                return fp
            np = np.next
            if np is None:
                return fp.next

            fp = fp.next



def test_mid():
    head = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5, None)))))

    assert Solution().middleNode(head).val == 3

    head = ListNode(0, head)
    assert Solution().middleNode(head).val == 3


if __name__ == '__main__':
    pytest.main(["./q876.py", "-s", "-v"])