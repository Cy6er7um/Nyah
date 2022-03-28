#[macro_export]
macro_rules! to_underscore {
    ($any:expr) => {_};
}

#[cfg(test)]
mod test {
    #[test]
    fn test_to_underscore() {
        // Here the macro is used to convert the string expression to an underscore.
        let a: to_underscore!("example") = 1;
        assert_eq!(a, 1);
    }
}
