use crate::common::list::ListNode;

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        return prev;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let node5 = ListNode::new(5);
        let mut node4 = ListNode::new(4);
        let mut node3 = ListNode::new(3);
        let mut node2 = ListNode::new(2);
        let mut head = ListNode::new(1);
        node4.next = Some(Box::new(node5));
        node3.next = Some(Box::new(node4));
        node2.next = Some(Box::new(node3));
        head.next = Some(Box::new(node2));

        let mut res = Solution::reverse_list(Some(Box::new(head)));

        for expected in [5, 4, 3, 2, 1] {
            let node = res.as_ref().unwrap();
            assert_eq!(node.val, expected);
            res = res.unwrap().next;
        }

        assert!(res.is_none());
    }
}
