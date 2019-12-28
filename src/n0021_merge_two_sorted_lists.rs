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
    pub fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        let mut result = vec![];
        while l1.is_some() && l2.is_some() {
            let v1 = l1.as_ref().unwrap().val;
            let v2 = l2.as_ref().unwrap().val;
            if v1 < v2 {
                result.push(v1);
                l1 = l1.unwrap().next;
            } else {
                result.push(v2);
                l2 = l2.unwrap().next;
            }
        }
        let rest = if l1.is_some() { l1 } else { l2 };
        let mut t = rest;
        for &curr in result.iter().rev() {
            let q = Box::new(ListNode{val: curr, next: t});
            t = Some(q);
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_two_sorted_lists() {
        let l1 = vec![1, 2, 4];
        let l2 = vec![1, 3, 4];
        let expect = vec![1, 1, 2, 3, 4, 4];
        assert_eq!(Solution::merge_two_lists(build(&l1), build(&l2)), build(&expect));
    }

    fn build(v: &Vec<i32>) -> Option<Box<ListNode>> {
        let mut t = None;
        for &curr in v.iter().rev() {
            let q = Box::new(ListNode{val:curr, next: t.take()});
            t = Some(q);
        }
        t
    }
}