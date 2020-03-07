use crate::list::ListNode;

struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut p = &mut head;
        while let Some(curr) = p {
            while let Some(next) = curr.next.as_mut() {
                if curr.val != next.val {
                    break;
                }

                let next_next = std::mem::replace(&mut next.next, None);
                std::mem::replace(&mut curr.next, next_next);
            }
            p = &mut curr.next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::linkedlist;

  #[test]
  fn test_83() {
    assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 2]), linkedlist![1, 2]);
    assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 2, 3, 3]), linkedlist![1, 2, 3]);
    assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 1]), linkedlist![1]);
  }
}