trait NyahObjectData: Clone {
    fn nyah_data_equal(&self, _other: &Self) -> bool {
        false
    }
}
