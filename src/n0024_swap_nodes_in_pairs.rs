use crate::list::ListNode;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(
            Solution::swap_pairs(linkedlist![1, 2, 3, 4]),
            linkedlist![2, 1, 4, 3]
        );
    }
}
