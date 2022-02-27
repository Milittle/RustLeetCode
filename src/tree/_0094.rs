// Definition for a binary tree node.
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

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
}

pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        Solution::inorder(&root, &mut res);
        res
    }
    pub fn inorder(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if root.is_none() {
            return;
        }
        Solution::inorder(&root.borrow().left, res);
        res.push(root.borrow().val);
        Solution::inorder(&root.borrow().right, res);
    }
}
