# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def deleteDuplicates(self, head: ListNode) -> ListNode:
        if head:
            v = head.val
            if head.next and head.next.val == v:
                while head and head.val == v:
                    head = head.next
                return self.deleteDuplicates(head)
            head.next = self.deleteDuplicates(head.next)
        return head   
