//#[cfg(test)]
pub mod math {
    pub const fn add(a: i8, b: i8) -> i8 {
        return a + b;
    }

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}