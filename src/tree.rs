
pub enum Tree<T> {
    Nil,
    Node(T, Vec<Tree<T>>)
}

impl<T> Tree<T> {
    pub fn leaf(val: T) -> Tree<T> {
        Node(val, vec!())
    }

    fn is_leaf(&self) -> bool {
        match *self {
            Node(_, ref vec) => {
                vec.len() == 0
            },
            Nil => false
        }
    }

    fn is_empty(&self) -> bool {
        match *self {
            Nil => true,
            _   => false
        }
    }
}
