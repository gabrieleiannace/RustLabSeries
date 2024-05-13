#[cfg(test)]
mod tests {
    use part1::list2::*;

    #[test]
    fn push_front_adds_element_to_empty_list() {
        let mut list = List::new();
        list.push_front(1);
        println!("{:?}", list);
    }

    #[test]
    fn push_front_adds_element_to_non_empty_list() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
    }

    #[test]
    fn test_pop_front() {
        let mut list = List::new();

        // Push some elements into the list
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        // Pop the elements and check
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));

        // At this point, the list should be empty
        assert_eq!(list.pop_front(), None);
    }
}