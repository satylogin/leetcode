// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::mem;

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut min: i32 = -1;
        for (i, list) in lists.iter().enumerate() {
            if list.is_some() {
                if min == -1 || list.as_deref().unwrap().val < lists[min as usize].as_deref().unwrap().val {
                    min = i as i32;
                }
            }
        }
        if min == -1 {
            None
        } else {
            let val = lists[min as usize].as_deref().unwrap().val;
            let mut x: Option<Box<ListNode>> = None;
            mem::swap(&mut lists[min as usize], &mut x);
            x = x.unwrap().next;
            mem::swap(&mut lists[min as usize], &mut x);
            Some(Box::new(ListNode {
                val,
                next: Solution::merge_k_lists(lists)
            }))
        }
    }
}
