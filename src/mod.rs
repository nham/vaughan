use std::iter::Peekable;

type Token = String;
type Stream<T> = Peekable<TokenObject, T>;

enum TokenObject {
    Literal(Tree<String>),
    Op(BinaryOp),
}


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

#[deriving(Clone)]
struct BinaryOp {
    lbp: uint,
    rbp: uint,
    op: String,
}

impl BinaryOp {
    fn render_subtree(self, child1: Tree<String>, child2: Tree<String>) -> Tree<String> {
        Node(self.op, vec!(child1, child2))
    }
}

// Takes a partial parse Tree, a current operator C, a previous operator, and 
// stream of tokens and returns the subtree with C as the root and the remaining
// stream of tokens
fn renderOp<T: Iterator<TokenObject>>(acc: Tree<String>, currOp: BinaryOp, prevOp: Option<BinaryOp>, mut stream: Stream<T>)
    -> Result<(Tree<String>, Stream<T>), &str> {

    let nextNonOp = match stream.next().unwrap() {
        Literal(t) => t,
        _          => { return Err("Invalid expression"); },
    };

    let bind_curr: bool;

    {
        bind_curr = match stream.peek() {
            None => { // we've exhausted the stream
                true
            },
            Some(obj) => { 
                match *obj {
                    Literal(_) => { // we've seen two Literals in a row, error
                        return Err("Invalid expression");
                    },
                    Op(ref op) => op.lbp <= currOp.rbp,
                }
            },
        };
    }

    if bind_curr {
        Ok((currOp.render_subtree(acc, nextNonOp), stream))
    } else {

    }

    Ok((Nil, stream))
    //Ok((currOp.render_subtree(acc, nextNonOp), stream));

}


// operators know how to render their parse trees. literals render leaves
fn main() {
    let t = Tree::leaf(55);

    println!("{}", t.is_leaf());
}

