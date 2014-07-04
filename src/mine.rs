use dlist::DList;
use std::collections::Deque;

use tree::Tree;

mod tree;
mod dlist;

type StrTree<'a> = Tree<&'a str>;
type Doodad<'a> = DList<StrTree<'a>>;

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
    fn partial_cmp(&self, other: & Operator<'a>) -> Option<Ordering> {
        if self.rep == "#" {
            Some(Less)
        } else if other.rep == "@" {
            Some(Greater)
        } else {
            Some(Equal)
        }
    }
}

impl<'a> Ord for Operator<'a> {
    fn cmp(&self, other: & Operator<'a>) -> Ordering {
        self.partial_cmp(other).unwrap()
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

    println!("{}", lst);

    opv.sort();
}
