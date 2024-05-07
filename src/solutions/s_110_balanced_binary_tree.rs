use std::cell::RefCell;
use std::rc::Rc;

use crate::common::treenode::TreeNode;

struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_balanced = true;

        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, is_balanced: &mut bool) -> i32 {
            match root {
                Some(node) => {
                    let left = dfs(&node.borrow().left, is_balanced);
                    let right = dfs(&node.borrow().right, is_balanced);

                    if (left - right).abs() > 1 {
                        *is_balanced = false;
                    }

                    return left.max(right) + 1;
                }
                None => return 0,
            }
        }

        dfs(&root, &mut is_balanced);
        return is_balanced;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root: TreeNode = TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        };

        let res = Solution::is_balanced(Some(Rc::new(RefCell::new(root))));
        assert!(res);
    }
}
