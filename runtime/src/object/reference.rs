use crate::interface::reference::NyahInterfaceReference;
use crate::util::pointer::NyahUtilPointer;

pub struct NyahObjectReference<D, const I: usize = 0> {
    pub pointer: NyahUtilPointer<D>,
    pub interfaces: [NyahInterfaceReference; I],
}

impl<D, const I: usize> NyahObjectReference<D, I>
{
    pub fn new(pointer: NyahUtilPointer<D>, interfaces: [NyahInterfaceReference; I]) -> Self {
        Self {
            pointer,
            interfaces,
        }
    }
}