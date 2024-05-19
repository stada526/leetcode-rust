use crate::common::list::ListNode;

struct Solution {}

impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        Self::reverse_without_nth(Self::reverse_without_nth(head, -1), n)
    }

    fn reverse_without_nth(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut cnt = 1;

        let mut pre: Option<Box<ListNode>> = None;
        let mut curr = head;
        while let Some(mut node) = curr {
            curr = node.next.take();
            if cnt != n {
                node.next = pre;
                pre = Some(node);
            }

            cnt += 1;
        }
        return pre;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 2;
        let head = ListNode::from(&input);
        let mut res = Solution::remove_nth_from_end(head, n);
        for expected in [1, 2, 3, 5] {
            assert_eq!(res.as_ref().unwrap().val, expected);
            res = res.unwrap().next;
        }
        assert!(res.is_none());
    }

    #[test]
    fn test_remove_nth() {
        let input = vec![1, 2, 3, 4, 5];
        let n = 3;
        let head = ListNode::from(&input);
        let mut res = Solution::reverse_without_nth(head, n);
        for expected in [5, 4, 2, 1] {
            assert_eq!(res.as_ref().unwrap().val, expected);
            res = res.unwrap().next;
        }
        assert!(res.is_none());
    }
}
