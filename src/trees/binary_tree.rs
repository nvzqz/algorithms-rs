/// A binary search tree.
pub struct BinaryTree<T> {
    data: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> From<T> for BinaryTree<T> {
    #[inline]
    fn from(val: T) -> BinaryTree<T> {
        BinaryTree { data: val, left: None, right: None }
    }
}

impl<T> BinaryTree<T> {
    /// Returns the data owned by this node.
    #[inline]
    pub fn data(&self) -> &T {
        &self.data
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
}
