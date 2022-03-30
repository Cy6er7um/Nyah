use crate::object::header::NyahObjectHeader;

pub struct NyahObjectModel<D: Clone> {
    pub header: NyahObjectHeader,
    pub data: D,
}
