pub mod list1 {
    use std::mem;


    pub enum ListLink<T> {
        Cons(T, Box<ListLink<T>>),
        Nil,
    }

    impl<T> Default for ListLink<T>{
        fn default() -> Self {
            Self::Nil
        }
    }
    pub struct List<T> {
        head: ListLink<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self { head: ListLink::Nil }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T) {
            let next_node = mem::take(&mut self.head);
            self.head = ListLink::Cons(elem, Box::new(next_node));    
        }

        pub fn pop(&mut self) -> Option<T> {
            let head = mem::take(&mut self.head);
            match head{
                ListLink::Cons(_elem, next_node) => {
                    self.head = *next_node;
                    return Some(_elem);
                },
                ListLink::Nil => {None},
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head{
                ListLink::Cons(_elem, _) => Some(_elem),
                ListLink::Nil => None,
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        pub fn iter(&self) -> ListIter<T> {
           ListIter { next: &self.head }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list: List<T> = List::new();
            for _ in 0..n {
                if let Some(value) = self.pop() {
                    new_list.push(value);
                } else {
                    panic!("Buffer overflow!");
                }
            }
            new_list
        }
    }


    pub struct ListIter<'a, T> {
       // implement the iterator trait for ListIter
       next: &'a ListLink<T>
    }
    
    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;
        
        fn next(&mut self) -> Option<Self::Item> {
            match &self.next{
                ListLink::Cons(elem, next_node) => {
                    self.next = &**next_node;
                    return Some(elem);
                },
                ListLink::Nil => None,
            }
        }   
    }

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T

}


pub mod list2 {

    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        // same methods as List1
        pub fn new() -> Self{
            List { head: None }
        }

        pub fn push(&mut self, elem: T){
            let head = self.head.take();
            let new_head = Some(Box::new(Node{
                elem,
                next: head,
            }));
            self.head = new_head;
        }

        pub fn pop(&mut self) -> Option<T>{
            let old_head = self.head.take();
            match old_head{
                Some(node) => {
                    self.head = node.next;
                    Some(node.elem)
                },
                None => None,
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head{
                Some(node) => Some(&node.elem),
                None => None,
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        pub fn iter(&self) -> ListIter<T> {
            ListIter { next: self.head.as_ref() }
        }

        
        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list: List<T> = List::new();
            for _ in 0..n {
                if let Some(value) = self.pop() {
                    new_list.push(value);
                } else {
                    panic!("Buffer overflow!");
                }
            }
            new_list
        }


    }

    pub struct ListIter<'a, T>{
        next: Option<&'a Box<Node<T>>>
    }

    impl<'a, T> Iterator for ListIter<'a, T>{
        type Item = &'a T;
    
        fn next(&mut self) -> Option<Self::Item> {
            let next_node = self.next.take()?;
            self.next = next_node.next.as_ref();
            return Some(&next_node.elem);
        }
    }
}

// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  youu can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore: 
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None

//  type NodeLink = Option<Rc<RefCell<DNode>>>; // we define a type alias for better readibility
// Example
//  type NodeBackLink = ... 

// struct DNode {
    // v: i32,
    // prev: NodeBackLink // here we can't put NodeLink to avoid a cycle reference, what do we use?
    // next: NodeLink 
// }

// struct DList {
    // head: NodeLink,
    // tail: NodeLink
// }

pub mod list3{
    use std::{borrow::Borrow, cell::{Ref, RefCell}, path::Iter, rc::{Rc, Weak}};

    //Definizione dei tipi:
    type NodeLink       =       Option<Rc<RefCell<DNode>>>;          //IN AVANTI
    type NodeBackLink   =       Option<Weak<RefCell<DNode>>>;        //IN INDIETRO

    #[derive(Debug)]
    pub struct DNode{
        v: i32,
        prev: NodeBackLink,
        next: NodeLink
    }

    #[derive(Debug)]
    pub struct Dlist{
        head: NodeLink,
        tail: NodeLink
    }

    impl Dlist{
        pub fn new() -> Self{
            Self { head: None, tail: None }
        }

        pub fn push_front(&mut self, elem: i32){
            let head_node = self.head.take();
            match head_node{
                Some(node) => {
                    // 1.	Creazione nodo con:
				    //                         i) v = elem
				    //                         ii) next = self.head
				    //                         iii) prev = None
                    // 2.	self.head = nodo
                    // 3.	settare nella old head il nodo appena creato come prev
                    let new_node = RefCell::new(DNode{
                        v: elem,
                        prev: None,
                        next: Some(node.clone()),
                    });
                    let rc_new_node = Rc::new(new_node);
                    self.head = Some(rc_new_node.clone());
                    let mut mut_old_head = (*node).borrow_mut();
                    mut_old_head.prev = Some(Rc::downgrade(&rc_new_node));
                    
                },
                None => {
                    //Se la lista è vuota:
                    //1.    creo nuovo elemento con v = elem
                    //2.    next e prev sono entrambi None
                    //3.    setto testa e coda sul nodo appena creato
                    let new_nodelink = Rc::new(RefCell::new(DNode{
                        v: elem,
                        prev: None,
                        next: None,
                    }));
                    self.head = Some(new_nodelink.clone());
                    self.tail = Some(new_nodelink.clone());
                },
            }
        }

        pub fn pop_front(&mut self) -> Option<i32> {            
            // Passaggi:
            // 1.	Salvarsi il next del nodo head
            // 2.	andare in head+1 e settare prev = None
            // 3.	ritornare v
            let head = self.head.take()?;
            let mut_head = (*head).borrow_mut();
            let next_head = (*mut_head).next.clone();
            match next_head{
                Some(next_head) => {
                    let mut mut_next_head = (*next_head).borrow_mut();
                    mut_next_head.prev = None;
                    self.head = Some(next_head.clone());
                    return Some(mut_head.v);
                },
                None => {
                    self.head = None;
                    self.tail = None;
                    return Some(mut_head.v)
                },
            }    
        }

        pub fn push_back(&mut self, item: i32){
            // Passaggi:
            // 1.	Creazione nuovo nodo
            //     1.1	v = item
            //     1.2	prev = self.tail
            //     1.3	tail = None
            // 2.	In oldTail mettere
            //     2.1	next = nodo
            // 3.	Settare il nodo come nuova tail            
            let tail_node = self.tail.take();
            match tail_node{
                Some(tail) => {
                    let weakprev = Rc::downgrade(&tail);
                    let node = DNode{
                        v: item,
                        prev: Some(weakprev),
                        next: None,
                    };

                    let mut mut_tail_node = (*tail).borrow_mut();
                    let rc_node = Rc::new(RefCell::new(node));
                    
                    mut_tail_node.next = Some(rc_node.clone());
                    self.tail = Some(rc_node.clone());
                },
                None => {
                    // Passaggi:
                    //     1.	Crezione nuovo nodo
                    //         1.1	v = item
                    //         1.2	next = None
                    //         1.3	prev = None
                    //     2.	Setta:
                    //             self.head = node
                    //             self.tail = node
                    let node = DNode{
                        v: item,
                        prev: None,
                        next: None,
                    };
                    let rc_node = Rc::new(RefCell::new(node));
                    self.head = Some(rc_node.clone());
                    self.tail = Some(rc_node.clone());
                },
            }
        }

        pub fn pop_back(&mut self) -> Option<i32>{
            // Passaggi:
            //         1.	Accedere alla tail corrente
            //         2.	Accedere a tail-1
            //             2.1	Settare il next = None 
            //         3.	Settare self.tail = tail-1
            //         4.	Ritornare tail.v
            let tail = self.tail.take();
            match tail{
                Some(tail_node) => {
                    //Accedere a tail-1
                    let tail = (*tail_node).borrow();
                    let new_tail = &tail.prev;
                    match new_tail{
                        Some(new_tail) => {
                            let new_tail = new_tail.upgrade().unwrap();
                            let mut mut_new_tail = (*new_tail).borrow_mut();
                            mut_new_tail.next = None;
                            self.tail = Some(new_tail.clone());
                            return Some(tail.v);
                        },
                        //Se è l'unico elemento allora lo restituisco e setto tutto a none
                        None => {
                            self.tail = None;
                            self.head = None;
                            return Some(tail.v);
                        },
                    }
                },
                None => return None,
            }
        }        

        fn popn(&mut self, n: usize) -> Option<i32>{
            //Passaggi:
                        // 1.  Arrivare al nodo n-esimo
            let mut cnode = self.head.clone();
            for _ in 0..n{
                if let Some(n) = cnode{
                    cnode = (*n).borrow().next.clone();
                }
                else {
                    return None
                }
            }

            cnode.and_then(|node| {
                let prev = (*node).borrow().prev.clone().map(|p| p.upgrade().unwrap());
                let next = (*node).borrow().next.clone();
                
                match (prev, next){
                    (None, None) => {
                        self.head = None;
                        self.tail = None;
                    },
                    (None, Some(next)) => {
                        self.head = Some(next.clone());
                        next.borrow_mut().prev = None;
                    },
                    (Some(prev), None) => {
                        self.tail = Some(prev.clone());
                        prev.borrow_mut().next = None;
                    },
                    (Some(prev), Some(next)) => {
                        prev.borrow_mut().next = Some(next.clone());
                        next.borrow_mut().prev = Some(Rc::downgrade(&prev));
                    },
                }

                Rc::try_unwrap(node)
                    .ok() // transform into an Option
                    .map(|x| x.into_inner().v) // transform into an Option<DNode<T>>
            })            
        }

        pub fn iter(&self) -> DListIter{
            todo!()
        }
    }

    pub struct DListIter{
        next: NodeLink
    }

    impl DListIter {
        pub fn new(list: Dlist) -> Self{
            Self{next: list.head}
        }
    }

    impl Iterator for DListIter{
        type Item = Rc<RefCell<DNode>>;
    
        fn next(&mut self) -> Option<Self::Item> {
            let cnode = self.next.clone();
            cnode.as_ref().map(|node| {
                let n = (**node).borrow();
                match &n.next {
                    Some(n) => {
                        self.next = Some(n.clone());
                    }
                    None => {
                        self.next = None;
                    }
                };
            });
            
            cnode
        }
    }

}