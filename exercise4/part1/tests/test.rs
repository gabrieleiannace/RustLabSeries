#[cfg(test)]
mod test1 {
    use ex1::list1::*;

    #[test]
    fn push_and_pop() {
        let mut list = List::new();

        // Check that an empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek_test() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
    }

    #[test]
    fn iter_test() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn take_test() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let taken_list = list.take(2);

        assert_eq!(taken_list.peek(), Some(&2));
        assert_eq!(list.peek(), Some(&1));
    }
}

#[cfg(test)]
mod test2{
    use ex1::list2::*;

    #[test]
    fn push_and_pop() {
        let mut list = List::new();

        // Verifica che una lista vuota si comporti correttamente
        assert_eq!(list.pop(), None);

        // Popola la lista
        list.push(1);
        list.push(2);
        list.push(3);

        // Verifica la rimozione normale
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Aggiunge altri elementi per assicurarsi che nulla sia corrotto
        list.push(4);
        list.push(5);

        // Verifica la rimozione normale
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Verifica l'esaurimento
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

}

#[cfg(test)]
mod test3{

    use ex1::list3::*;
    
    #[test]
    fn test_push_front_and_pop_front_single_element() {
        let mut list = Dlist::new();
        list.push_front(10);
        let popped = list.pop_front();
        assert_eq!(popped, Some(10));
    }

    #[test]
    fn test_push_front_and_pop_front_multiple_elements() {
        let mut list = Dlist::new();
        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        let popped1 = list.pop_front();
        assert_eq!(popped1, Some(30));

        let popped2 = list.pop_front();
        assert_eq!(popped2, Some(20));

        let popped3 = list.pop_front();
        assert_eq!(popped3, Some(10));
    }

    #[test]
    fn test_push_back() {
        let mut list = Dlist::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
    }

    #[test]
    fn test_pop_back() {
        let mut list = Dlist::new();
        assert_eq!(list.pop_back(), None);
        list.push_back(1);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_push_and_pop_back() {
        let mut list = Dlist::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }
}