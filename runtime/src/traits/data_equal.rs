pub trait NyahDataEqual {
    fn nyah_data_equal(&self, _other: &Self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::traits::data_equal::NyahDataEqual;

    struct Example;

    impl NyahDataEqual for Example {}

    #[test]
    fn test_default_implement() {
        let instance = Example;
        assert!(!instance.nyah_data_equal(&instance));
    }
}
