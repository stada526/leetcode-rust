use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from(vec: &Vec<Option<i32>>) -> Option<Self> {
        let root_node = match vec[0] {
            Some(val) => TreeNode::new(val),
            None => return None,
        };

        if vec.len() == 1 {
            return Some(root_node);
        }

        let root = Rc::new(RefCell::new(root_node));

        let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
        queue.push_back(Some(Rc::clone(&root)));

        for index in 1..vec.len() {
            let is_right_node = index % 2 == 0;

            let node_val = vec[index];
            match node_val {
                Some(val) => {
                    let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
                    queue.push_back(Some(Rc::clone(&new_node)));
                    let parent_node_ref = &mut queue[0];

                    if let Some(parent_node) = parent_node_ref {
                        if is_right_node {
                            parent_node.borrow_mut().right = Some(new_node);
                        } else {
                            parent_node.borrow_mut().left = Some(new_node);
                        }
                    }
                }
                None => {
                    queue.push_back(None);
                }
            }

            if is_right_node {
                queue.pop_front();
            }
        }

        return Some(Rc::try_unwrap(root).unwrap().into_inner());
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;

    #[test]
    fn test_single_element() {
        let input = vec![Some(1)];

        let root = TreeNode::from(&input);

        assert_eq!(root.as_ref().unwrap().val, 1);
    }

    #[test]
    fn test_multiple_elements() {
        let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];

        let root = TreeNode::from(&input);

        let node_9 = root.as_ref().unwrap().left.as_ref().unwrap().borrow();
        let node_20 = root.as_ref().unwrap().right.as_ref().unwrap().borrow();
        let node_15 = node_20.left.as_ref().unwrap().borrow();
        let node_7 = node_20.right.as_ref().unwrap().borrow();

        // assert
        assert_eq!(root.as_ref().unwrap().val, 3);
        assert_eq!(node_9.val, 9);
        assert!(node_9.left.is_none());
        assert!(node_9.right.is_none());
        assert_eq!(node_20.val, 20);
        assert_eq!(node_15.val, 15);
        assert_eq!(node_7.val, 7);
        assert!(node_7.left.is_none());
        assert!(node_7.right.is_none());
    }
}
