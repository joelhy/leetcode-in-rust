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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        use std::mem;

        let mut head = head;
        let mut n1 = &mut head;
        while n1.is_some() && n1.as_ref().unwrap().next.is_some() {
            let mut n2 = n1.as_mut().unwrap().next.take();
            mem::swap(&mut n1.as_mut().unwrap().next, &mut n2.as_mut().unwrap().next);
            mem::swap(&mut n2.as_mut().unwrap().next, &mut n1);
            mem::swap(n1, &mut n2);
            n1 = &mut n1.as_mut().unwrap().next.as_mut().unwrap().next;
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
    fn test() {
        assert_eq!(
            Solution::swap_pairs(linkedlist![1, 2, 3, 4]),
            linkedlist![2, 1, 4, 3]
        );
    }
}
