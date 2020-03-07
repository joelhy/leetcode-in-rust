use crate::list::ListNode;

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut len = 0;
        {
            let mut curr = dummy.as_ref();
            while curr.unwrap().next.is_some() {
                len += 1;
                curr = curr.unwrap().next.as_ref();
            }
        }

        let idx = len - n;
        let mut new_curr = dummy.as_mut();
        for _ in 0..idx {
            new_curr = new_curr.unwrap().next.as_mut();
        }

        if let Some(curr) = new_curr {
            if let Some(next) = curr.next.as_mut() {
                let next_next = next.next.take();
                curr.next = next_next;
            }
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::linkedlist;

    #[test]
    fn test() {
        assert_eq!(
            Solution::remove_nth_from_end(linkedlist![1, 2, 3, 4, 5], 2),
            linkedlist![1, 2, 3, 5]
        );
    }
}

