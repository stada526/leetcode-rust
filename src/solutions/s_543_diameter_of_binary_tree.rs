use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

use crate::common::solution::Solution;
use crate::common::treenode::TreeNode;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max = 0;

        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> i32 {
            match root {
                None => 0,
                Some(node) => {
                    let ref left = node.borrow().left;
                    let ref right = node.borrow().right;
                    let left_length = dfs(left, max);
                    let right_length = dfs(right, max);

                    *max = cmp::max(*max, left_length + right_length);

                    return left_length.max(right_length) + 1;
                }
            }
        }

        dfs(&root, &mut max);

        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root: TreeNode = TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        };

        let res = Solution::diameter_of_binary_tree(Some(Rc::new(RefCell::new(root))));

        assert_eq!(res, 3);
    }
}
