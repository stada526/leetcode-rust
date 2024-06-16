struct Solution {}

use std::cell::RefCell;
use std::rc::Rc;

use crate::common::treenode::TreeNode;
impl Solution {
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        return Self::dfs(&root, i32::MIN as i64 - 1, i32::MAX as i64 + 1);
    }
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, min: i64, max: i64) -> bool {
        match root {
            Some(node) => {
                if !(min < node.borrow().val as i64 && (node.borrow().val as i64) < max) {
                    return false;
                }
                return Self::dfs(&node.borrow().left, min, node.borrow().val as i64)
                    && Self::dfs(&node.borrow().right, node.borrow().val as i64, max);
            }
            None => return true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let node = TreeNode::from(&vec![Some(2), Some(1), Some(3)]).unwrap();
        let res = Solution::is_valid_bst(Some(Rc::new(RefCell::new(node))));
        assert!(res)
    }
    #[test]
    fn test_invalid() {
        let node = TreeNode::from(&vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ])
        .unwrap();
        let res = Solution::is_valid_bst(Some(Rc::new(RefCell::new(node))));
        assert!(!res)
    }
}
