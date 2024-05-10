#[cfg(test)]
mod tests {
    use part1::List2::List;

    #[test]
    fn new() {
        let list: List<i32> = List::new();
        assert!(list.peek().is_none());
    }

    #[test]
    fn push_and_pop() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert!(list.peek().is_none());
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek().is_none());
        list.push(2);
        assert_eq!(list.peek(), Some(&2));
    }


    #[test]
    fn take() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        
        let mut taken_list = list.take(2);
        assert!(taken_list.pop().is_some());
        assert!(taken_list.pop().is_some());
    }
}
