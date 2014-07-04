use std::collections::Deque;
use std::ptr;
use std::mem;
use std::fmt;

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    // [a ->] [<- b ->] [<- c ->]
    // in the crude diagram above, a.next points to b, as does c.prev
    // but we're using owned pointers here, so obviously a.next and c.prev
    // can't both own b. we have the next pointers be owning pointers. prev
    // will just be raw
    prev: *mut Node<T>,
    next: Link<T>,
    value: T,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node { prev: ptr::mut_null(), next: None, value: value }
    }
}

pub struct DList<T> {
    length: uint,
    front: Link<T>,
    back: *mut Node<T>,
}

impl<T> Mutable for DList<T> {
    fn clear(&mut self) {
        *self = DList::new();
    }
}

impl<T> Collection for DList<T> {
    fn len(&self) -> uint {
        self.length
    }

    fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T> DList<T> {
    pub fn new() -> DList<T> {
        DList { front: None, back: ptr::mut_null(), length: 0 }
    }
}

impl<T> Deque<T> for DList<T> {
    fn front<'a>(&'a self) -> Option<&'a T> {
        match self.front {
            None       => None,
            Some(ref node) => Some(&'a node.value),
        }
    }

    fn front_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        match self.front {
            None       => None,
            Some(ref mut node) => Some(&'a mut node.value),
        }
    }

    fn back<'a>(&'a self) -> Option<&'a T> {
        unsafe {
            match self.back.to_option() {
                None     => None,
                Some(ref node) => Some(&'a node.value),
            }
        }

    }


    /*

    std::collections::DList does this like this:

    fn back_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        self.list_tail.resolve().map(|tail| &mut tail.value)
    }

    fn resolve(&mut self) -> Option<&mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

     */

    fn back_mut<'a>(&'a mut self) -> Option<&'a mut T> {
        if self.back.is_null() {
            None
        } else {
            Some(unsafe { &'a mut (*self.back).value })
        }
    }

    fn push_front(&mut self, elt: T) {
        let mut new = box Node::new(elt);
        if self.front.is_none() {
            self.back = &mut *new as *mut Node<T>;
            self.front = Some(new);
        } else {
            let old_front = self.front.take();
            new.next = old_front;
            self.front = Some(new);

            // I think we need to replace self.front with new, and then set
            // new.next to the thing we replaced
        }

        self.length += 1;
    }

    fn push_back(&mut self, elt: T) {
        let mut new = box Node::new(elt);

        unsafe {
            match self.back.to_option() {
                None => {
                    self.back = &mut *new as *mut Node<T>;
                    self.front = Some(new);
                },
                Some(ref node) => {
                    new.prev = self.back;
                    let new_ptr = &mut *new as *mut Node<T>;
                    (*self.back).next = Some(new);
                    self.back = new_ptr;
                }
            }
        }

        self.length += 1;
    }

    /* from std::collections

     /// A doubly-linked list.
    pub struct DList<T> {
        length: uint,
        list_head: Link<T>,
        list_tail: Rawlink<Node<T>>,
    }

    type Link<T> = Option<Box<Node<T>>>;
    struct Rawlink<T> { p: *mut T }

 
    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.list_tail.resolve().map_or(None, |tail| {
            self.length -= 1;
            self.list_tail = tail.prev;
            match tail.prev.resolve() {
                None => self.list_head.take(),
                Some(tail_prev) => tail_prev.next.take()
            }
        })
    }

    ahh, actually, Rawlink.p is a *mut Node<T>, so we're calling the is_null()
    for *mut T

    fn resolve(&mut self) -> Option<&mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }


    mem::transmute might just return the same value under a new type. the return
    type is inferred here by the return type of resolve(). so we have an *mut T (self.p) that we are transmuting to &mut T.

impl<T> RawPtr<T> for *mut T {
    #[inline]
    fn null() -> *mut T { mut_null() }

    #[inline]
    fn is_null(&self) -> bool { *self == RawPtr::null() }


    RawPtr::null for *mut T calls ptr::mut_null():

    pub fn mut_null<T>() -> *mut T { 0 as *mut T }

    pub fn null<T>() -> *const T { 0 as *const T }

    /// Takes the value out of the option, leaving a `None` in its place.
    #[inline]
    pub fn take(&mut self) -> Option<T> {
        mem::replace(self, None)
    }

    /// Remove the first Node and return it, or None if the list is empty
    #[inline]
    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        self.list_head.take().map(|mut front_node| {
            self.length -= 1;
            match front_node.next.take() {
                Some(node) => self.list_head = link_with_prev(node, Rawlink::none()),
                None => self.list_tail = Rawlink::none()
            }
            front_node
        })
    }


    pub fn replace<T>(dest: &mut T, mut src: T) -> T {
        swap(dest, &mut src);
        src
    }

    there are two swaps. mem::swap (the one mem::replace uses) and ptr::swap. They're actually different.

    pub fn swap<T>(x: &mut T, y: &mut T) {
        unsafe {
            // Give ourselves some scratch space to work with
            let mut t: T = uninitialized();

            // Perform the swap, `&mut` pointers never alias
            ptr::copy_nonoverlapping_memory(&mut t, &*x, 1);
            ptr::copy_nonoverlapping_memory(x, &*y, 1);
            ptr::copy_nonoverlapping_memory(y, &t, 1);

            // y and t now point to the same thing, but we need to completely forget `t`
            // because it's no longer relevant.
            forget(t);
        }
    }

    Notice the calls to copy_nonoverlapping_memory. This is the equivalent of libc's memcpy, meaning we assume 
    the addresses never overlap.

    pub unsafe fn swap<T>(x: *mut T, y: *mut T) {
        // Give ourselves some scratch space to work with
        let mut tmp: T = mem::uninitialized();
        let t: *mut T = &mut tmp;

        // Perform the swap
        copy_nonoverlapping_memory(t, &*x, 1);
        copy_memory(x, &*y, 1); // `x` and `y` may overlap
        copy_nonoverlapping_memory(y, &*t, 1);

        // y and t now point to the same thing, but we need to completely forget `tmp`
        // because it's no longer relevant.
        mem::forget(tmp);
    }

    AHA! copy_memory() spotted. This is the equivalent of libc's memmove, which does handle overlapping memory

    The difference is in the arguments. ptr::swap handles *mut T's, while mem::swap takes &mut T's.

    ######

    pub unsafe fn copy_nonoverlapping_memory<T>(dst: *mut T, src: *T, count: uint)

    ######

    Option::map() -

    pub fn map<U>(self, f: |T| -> U) -> Option<U> {
        match self { Some(x) => Some(f(x)), None => None }
    }

    */

    fn pop_back(&mut self) -> Option<T> {
        match mut_raw_to_mut_ref(self.back) {
            None => None, // Nothing to pop
            Some(curr_tail) => {
                self.back = curr_tail.prev;
                self.length -= 1;

                Some(
                    match mut_raw_to_mut_ref(curr_tail.prev) {
                        None => unwrap_link( self.front.take() ) ,
                        Some(tail_prev) => unwrap_link( tail_prev.next.take() ),
                    })
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        // if I'm taking the value in self.front out and leaving None in its place, 
        // don't I need to replace it with self.front.take().next?
        match self.front.take() {
            None => None,  // nothing to pop.
            Some(mut f) => {
                self.length -= 1;

                match f.next.take() {
                    None => { 
                        // if front.next was None, we only had 1 node in the list
                        // so null out self.back
                        self.back = ptr::mut_null(); 
                    },
                    Some(node) => {
                        self.front = Some(node);
                    },
                }

                Some(f.value)
            }
        }
    }
}


// Not completely safe. We do check if the pointer is null (and return None 
// accordingly), but if the pointer is non-null and still invalid there will be
// a problem.
fn mut_raw_to_mut_ref<T>(ptr: *mut T) -> Option<&mut T> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { mem::transmute(ptr) })
    }
}


// will fail if it's None
fn unwrap_link<T>(node: Link<T>) -> T {
    (*node.unwrap()).value
}


impl<A: fmt::Show> fmt::Show for DList<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[");

        let mut link = &self.front;
        while !link.is_none() {
            let node: &Node<A> = &**(link.get_ref());
            write!(f, "{}", node.value);
            link = &node.next;
        }

        write!(f, "]")
    }
}
