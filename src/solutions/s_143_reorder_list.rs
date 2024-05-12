use crate::common::list::ListNode;

struct Solution {}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut second_half_head = Solution::get_second_half_head(head);
        let mut reversed_head = Solution::reverse(&mut second_half_head);

        Solution::merge(head, &mut reversed_head);
    }

    fn get_second_half_head(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head.clone();
        let mut slow = head;

        fast = match fast {
            Some(node) => &node.next,
            None => return None,
        };

        loop {
            match (slow, fast) {
                (None, None) => return None,
                (None, Some(_)) => panic!(),
                (Some(node), None) => return node.next.take(),
                (Some(slow_node), Some(fast_node)) => {
                    fast = &fast_node.next;
                    fast = match fast {
                        Some(fast_node) => &fast_node.next,
                        None => return slow_node.next.take(),
                    };
                    slow = &mut slow_node.next;
                }
            }
        }
    }

    fn reverse(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.take();
        let mut pre: Option<Box<ListNode>> = None;
        while let Some(mut node) = curr {
            curr = node.next.take();
            node.next = pre;
            pre = Some(node);
        }
        return pre;
    }

    fn merge(head1: &mut Option<Box<ListNode>>, head2: &mut Option<Box<ListNode>>) {
        let mut h1 = head1;
        let mut h2 = head2.take();
        while h1.is_some() && h2.is_some() {
            let h1_next = h1.as_mut().unwrap().next.take();
            let h2_next = h2.as_mut().unwrap().next.take();

            h2.as_mut().unwrap().next = h1_next;
            h1.as_mut().unwrap().next = h2.take();

            h1 = &mut h1.as_mut().unwrap().next.as_mut().unwrap().next;
            h2 = h2_next;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 2, 3, 4, 5];
        let mut head = ListNode::from(&input);
        Solution::reorder_list(&mut head);

        for expected in [1, 5, 2, 4, 3] {
            assert_eq!(head.as_ref().unwrap().val, expected);
            head = head.unwrap().next;
        }

        assert!(head.is_none());
    }

    #[test]
    fn test_get_second_half_head() {
        let input = vec![1, 2, 3, 4, 5];
        let mut head = ListNode::from(&input);

        let mut res = &Solution::get_second_half_head(&mut head);

        for expected in [4, 5] {
            assert_eq!(res.as_ref().unwrap().val, expected);
            res = &res.as_ref().unwrap().next;
        }

        assert!(res.as_ref().is_none());
    }

    #[test]
    fn test_reverse() {
        let input = vec![1, 2, 3];
        let mut head = ListNode::from(&input);

        let mut res = &Solution::reverse(&mut head);

        for expected in [3, 2, 1] {
            assert_eq!(res.as_ref().unwrap().val, expected);
            res = &res.as_ref().unwrap().next;
        }

        assert!(res.is_none());
    }
}
