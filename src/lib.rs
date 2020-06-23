#[cfg(test)]
mod geras_db {
    pub fn add(a: i8, b: i8) -> i8 {
        return a + b;
    }

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
