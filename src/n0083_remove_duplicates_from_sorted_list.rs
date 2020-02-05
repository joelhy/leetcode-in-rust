// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

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

macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new(ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new(ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head; // 避免 `unused_assignments`
            head.next
        }
    };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_83() {
    assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 2]), linkedlist![1, 2]);
    assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 2, 3, 3]), linkedlist![1, 2, 3]);
    assert_eq!(Solution::delete_duplicates(linkedlist![1, 1, 1]), linkedlist![1]);
  }
}