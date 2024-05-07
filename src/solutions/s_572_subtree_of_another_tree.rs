use std::cell::RefCell;
use std::rc::Rc;

use crate::common::treenode::TreeNode;
struct Solution {}

impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut stack = vec![];

        match root {
            Some(ref root_node) => {
                stack.push(Rc::clone(root_node));
            }
            None => {
                if sub_root.is_none() {
                    return true;
                }
            }
        }

        while stack.len() != 0 {
            let node = stack.pop().unwrap();
            if Solution::is_same_tree(&Some(Rc::clone(&node)), &sub_root) {
                return true;
            }

            let node_ref = node.borrow();

            let left = node_ref.left.as_ref();
            if left.is_some() {
                stack.push(Rc::clone(&left.unwrap()))
            }

            let right = node_ref.right.as_ref();
            if right.is_some() {
                stack.push(Rc::clone(&right.unwrap()))
            }
        }
        return false;
    }

    fn is_same_tree(
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (root1, root2) {
            (None, None) => return true,
            (None, Some(_)) => return false,
            (Some(_), None) => return false,
            (Some(ref node1), Some(ref node2)) => {
                let node1_ref = node1.borrow();
                let ref node1_left = node1_ref.left;
                let ref node1_right = node1_ref.right;

                let node2_ref = node2.borrow();
                let ref node2_left = node2_ref.left;
                let ref node2_right = node2_ref.right;

                if node1_ref.val != node2_ref.val {
                    return false;
                }

                return Solution::is_same_tree(node1_left, node2_left)
                    && Solution::is_same_tree(node1_right, node2_right);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let root = Rc::new(RefCell::new(
            TreeNode::from(&vec![Some(3), Some(4), Some(5), Some(1), Some(2)]).unwrap(),
        ));
        let sub_root = Rc::new(RefCell::new(
            TreeNode::from(&vec![Some(4), Some(1), Some(2)]).unwrap(),
        ));

        let res = Solution::is_subtree(Some(root), Some(sub_root));
        assert!(res)
    }

    #[test]
    fn test_ng() {
        let root = Rc::new(RefCell::new(TreeNode::from(&vec![Some(1)]).unwrap()));
        let sub_root = Rc::new(RefCell::new(TreeNode::from(&vec![Some(0)]).unwrap()));

        let res = Solution::is_subtree(Some(root), Some(sub_root));
        assert!(!res)
    }
}
