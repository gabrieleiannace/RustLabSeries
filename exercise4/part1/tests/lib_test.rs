#[cfg(test)]
mod tests {
    use part1::List1::List;

    #[test]
    fn test_push() {
        let mut list = List::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
    }
}