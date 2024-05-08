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
        let head = ListNode::from(&vec![1, 2, 3, 4, 5]);

        let mut res = Solution::reverse_list(head);

        for expected in [5, 4, 3, 2, 1] {
            let node = res.as_ref().unwrap();
            assert_eq!(node.val, expected);
            res = res.unwrap().next;
        }

        assert!(res.is_none());
    }
}
