mod shapes;

#[cfg(test)]
mod tests {

    use super::shapes::Rectangle;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_panic() {
        // panic!("fuck you");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let a = Rectangle {
            length: 10,
            width: 43
        };

        let b = Rectangle {
            length: 3,
            width: 3
        };

        assert!(a.can_hold(&b));
    }
}
