"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Node') -> 'Node':
        cur = head
        while cur:
            cur.next = Node(cur.val, cur.next, cur.random)
            cur = cur.next.next
        cur = head
        while cur:
            cur.next.random = cur.random.next if cur.random else None
            cur = cur.next.next
        cur = head
        new_head = cur.next if cur else None
        new_cur = new_head
        while cur:
            cur.next = cur.next.next
            new_cur.next = new_cur.next.next if new_cur.next else None
            cur = cur.next
            new_cur = new_cur.next
        return new_head
