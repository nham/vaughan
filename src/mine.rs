use std::collections::DList;
use std::collections::Deque;

type StrTree<'a> = Tree<&'a str>;
type Doodad<'a> = DList<StrTree<'a>>;

enum Tree<T> {
    Nil,
    Node(T, Vec<Tree<T>>)
}

impl<T> Tree<T> {
    fn leaf(val: T) -> Tree<T> {
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

#[deriving(PartialEq, Eq)]
struct Operator<'a> {
    rep: &'a str,
}

impl<'a> Operator<'a> {
    fn new<'a>(rep: &'a str) -> Operator<'a> {
        Operator { rep: rep }
    }
}


// # < $ < @
impl<'a> PartialOrd for Operator<'a> {
    fn lt(&self, other: & Operator<'a>) -> bool {
        self.rep == "#"
        || other.rep == "@"
    }
}

impl<'a> Ord for Operator<'a> {
    fn cmp(&self, other: & Operator<'a>) -> Ordering {
        if self.rep == other.rep {
            Equal
        } else if self < other {
            Less
        } else {
            Greater
        }
    }
}


fn is_op(s: &str) -> bool {
    s == "#" || s == "@" || s == "$"
}

impl<'a> Ord for *const StrTree<'a> {
    fn cmp(&self, other: &(*const StrTree<'a>)) -> Ordering {
        Equal
    }
}

unsafe fn reduce_highest<'a>(mut lst: Doodad<'a>, mut ops: Vec<(Operator, *const Tree<&str>)>) 
    -> DList<StrTree<'a>> {
        if lst.len() == 1 {
            return lst;
        }

        match ops.pop() {
            None => fail!("Something wrong happened"),
            Some((_, ptr)) => { return lst; }
        }
}



fn main() {
    let inp = vec!("a", "#", "b", "@", "c", "$", "d");
    let mut ops = vec!();
    ops.push(Operator::new("#"));
    ops.push(Operator::new("@"));
    ops.push(Operator::new("$"));


    let mut lst = DList::new();

    let mut opv = Vec::new();

    for token in inp.iter() {
        lst.push_back(Tree::leaf(*token));

        if is_op(*token) {
            let llref = lst.back().unwrap() as *const Tree<&str>;
            opv.push((Operator::new(*token), llref));
            //lst.back().unwrap() as *const Tree<&str>;
        }
    }

    opv.sort();
}
