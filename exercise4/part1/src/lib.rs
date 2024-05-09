
pub mod List1 {
    use std::mem;


    #[derive(Debug)]
    pub enum ListLink<T> {
        Cons(T, Box<ListLink<T>>),
        Nil,
    }

    #[derive(Debug)]
    pub struct List<T> {
        head: ListLink<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self{
                head: ListLink::Nil,
            }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T) {
            //Vecchia testa
            let old_head = mem::replace(&mut self.head, ListLink::Nil);
            self.head = ListLink::Cons(elem, Box::new(old_head));
        }

        fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, ListLink::Nil){
                ListLink::Nil => {return None},
                ListLink::Cons(e, l) => {
                    self.head = *l;
                    return Some(e);
                }
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                ListLink::Nil => None,
                ListLink::Cons(element, _) => Some(element)
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter{
                next: &self.head,
            }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut new_list = List::new();
            for _ in 0..n {
                if let Some(element) = self.pop(){
                    new_list.push(element);
                }
            }
            new_list
        }
    }


    struct ListIter<'a, T> {
       // implement the iterator trait for ListIter
       next: &'a ListLink<T>
    }
    
    impl<'a, T> Iterator for ListIter<'a, T> {
        //Poichè vogliamo restituire i valori nella lista, non i nodi nella lista
       type Item = &'a T;
    
       fn next(&mut self) -> Option<Self::Item> {
           match &self.next{
            ListLink::Nil => None,
            ListLink::Cons(e, n) => {
                self.next = n;
                Some(e)
            }
           }    
       }
    }

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T

}

pub mod List2 {

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

