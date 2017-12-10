/// A binary tree.
pub struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> From<T> for BinaryTree<T> {
    #[inline]
    fn from(value: T) -> BinaryTree<T> {
        BinaryTree { value: value, left: None, right: None }
    }
}

impl<T> BinaryTree<T> {
    /// Returns the data owned by this node.
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the left binary tree node.
    #[inline]
    pub fn left(&self) -> Option<&BinaryTree<T>> {
        self.left.as_ref().map(AsRef::as_ref)
    }

    /// Returns the right binary tree node.
    #[inline]
    pub fn right(&self) -> Option<&BinaryTree<T>> {
        self.right.as_ref().map(AsRef::as_ref)
    }

    /// Traverses the tree in-order.
    pub fn in_order<F: FnMut(&T)>(&self, mut f: F) {
        in_order_imp(self, &mut f);
    }

    /// Traverses the tree pre-order.
    pub fn pre_order<F: FnMut(&T)>(&self, mut f: F) {
        pre_order_imp(self, &mut f);
    }

    /// Traverses the tree post-order.
    pub fn post_order<F: FnMut(&T)>(&self, mut f: F) {
        post_order_imp(self, &mut f);
    }
}

// Implementations /////////////////////////////////////////////////////////////

fn in_order_imp<'a, T, F: FnMut(&'a T)>(tree: &'a BinaryTree<T>, f: &mut F) {
    tree.left.as_ref().map(|val| in_order_imp(val, f));
    f(&tree.value);
    tree.right.as_ref().map(|val| in_order_imp(val, f));
}

fn pre_order_imp<'a, T, F: FnMut(&'a T)>(tree: &'a BinaryTree<T>, f: &mut F) {
    f(&tree.value);
    tree.left.as_ref().map(|val| pre_order_imp(val, f));
    tree.right.as_ref().map(|val| pre_order_imp(val, f));
}

fn post_order_imp<'a, T, F: FnMut(&'a T)>(tree: &'a BinaryTree<T>, f: &mut F) {
    tree.left.as_ref().map(|val| post_order_imp(val, f));
    tree.right.as_ref().map(|val| post_order_imp(val, f));
    f(&tree.value);
}
