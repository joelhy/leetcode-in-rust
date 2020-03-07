use crate::list::ListNode;

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let (mut p, mut q, mut carry) = (l1, l2, 0);
        let mut current = head.as_mut();
        while p.is_some() || q.is_some() {
            let mut sum = carry;
            if let Some(x) = p {
                sum += x.val;
                p = x.next;
            }
            if let Some(y) = q {
                sum += y.val;
                q = y.next;
            }
            carry = sum / 10;
            if let Some(curr) = current {
                curr.next = Some(Box::new(ListNode::new(sum % 10)));
                current = curr.next.as_mut();
            }
        }
        if carry > 0 {
            current.unwrap().next = Some(Box::new(ListNode::new(carry)));
        }

        head.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn test_2() {
        let l1 = linkedlist![2, 4, 3];
        let l2 = linkedlist![5, 6, 4];
        assert_eq!(Solution::add_two_numbers(l1, l2), linkedlist![7, 0, 8]);
    }
}
