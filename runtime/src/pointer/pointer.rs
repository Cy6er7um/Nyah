#[repr(C)]
pub struct NyahUtilPointer<T> {
    pub pointer: *mut T,
}

impl<T> Clone for NyahUtilPointer<T> {
    fn clone(&self) -> Self {
        NyahUtilPointer {
            // Copy the pointer here.
            pointer: self.pointer,
        }
    }
}

impl<T> Copy for NyahUtilPointer<T> {}