mod node;
use node::Node;

#[derive(Debug, Clone)]
pub struct Trie<T> {
    pub(crate) children: Vec<Node<T>>,
    child_size: usize,
    seq: Vec<T>,
}
impl<T> Default for Trie<T> {
    fn default() -> Self {
        Self {
            children: vec![],
            child_size: 0,
            seq: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
