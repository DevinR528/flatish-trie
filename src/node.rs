#[derive(Debug, Clone, Eq)]
pub(crate) struct Node<T> {
    pub(crate) val: T,
    children: Vec<Node<T>>,
    child_size: usize,
    terminal: bool,
    index: usize,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T: Eq> Node<T> {
    pub(crate) fn new(val: T, index: usize, terminal: bool) -> Node<T> {
        Self {
            val,
            children: vec![],
            child_size: 0,
            terminal,
            index,
        }
    }
}
