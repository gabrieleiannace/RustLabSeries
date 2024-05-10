#[cfg(test)]
mod tests {
    use part1::List2::List;    

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
}