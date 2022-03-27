trait NyahObjectData: Clone {
    fn nyah_data_equal(&self, _other: &Self) -> bool {
        false
    }
}

#[cfg(test)]
mod test {
    use crate::traits::object_data::NyahObjectData;

    #[derive(Clone)]
    struct Example;

    impl NyahObjectData for Example {}

    #[test]
    fn test_default_implement() {
        let instance = Example;
        let compare_result = instance.nyah_data_equal(&instance.clone());
        assert_eq!(compare_result, false);
    }
}