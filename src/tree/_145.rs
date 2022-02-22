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
}

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn preorder(root: Option<&Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            match root {
                None => {}
                Some(x) => {
                    preorder(x.borrow().left.as_ref(), res);
                    preorder(x.borrow().right.as_ref(), res);
                    res.push(x.borrow().val);
                }
            }
        }
        let mut res = vec![];
        preorder(root.as_ref(), &mut res);
        res
    }
}
