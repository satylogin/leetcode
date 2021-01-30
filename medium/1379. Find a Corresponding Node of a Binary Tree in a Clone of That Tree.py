# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, x):
#         self.val = x
#         self.left = None
#         self.right = None

from queue import Queue

class Solution:
    def getTargetCopy(self, original: TreeNode, cloned: TreeNode, target: TreeNode) -> TreeNode:
        queue: Queue = Queue()
        queue.put((original, cloned))
        while not queue.empty():
            original, cloned = queue.get()
            if original:
                if original is target:
                    return cloned
                queue.put((original.left, cloned.left))
                queue.put((original.right, cloned.right))
        
        
        
