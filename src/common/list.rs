#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from(vec: &Vec<i32>) -> Option<Box<Self>> {
        let mut next_node = None;
        for val in vec.iter().rev() {
            let mut new_node = ListNode::new(*val);
            new_node.next = next_node.take();
            next_node = Some(Box::new(new_node));
        }
        return match next_node {
            Some(node) => Some(Box::new(*node)),
            None => None,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec![1, 2, 3, 4];

        let mut head = ListNode::from(&input);

        for expected in [1, 2, 3, 4] {
            assert_eq!(head.as_ref().unwrap().val, expected);
            head = head.unwrap().next;
        }

        assert!(head.is_none());
    }
}
