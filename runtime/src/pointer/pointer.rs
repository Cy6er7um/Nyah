#[repr(C)]
pub struct NyahUtilPointer<T> {
    pub pointer: *mut T,
}

impl<T> NyahUtilPointer<T> {
    pub fn new(pointer: *mut T) -> NyahUtilPointer<T> {
        NyahUtilPointer {
            pointer,
        }
    }
}

impl<T> Clone for NyahUtilPointer<T> {
    fn clone(&self) -> Self {
        NyahUtilPointer {
            // Copy the pointer here.
            pointer: self.pointer,
        }
    }
}

// If you use derive to automatically implement Copy,
// then Pointer will not be able to Copy when T does not implement Copy.
impl<T> Copy for NyahUtilPointer<T> {}

#[cfg(test)]
mod test {
    use crate::pointer::pointer::NyahUtilPointer;

    struct Example;

    #[test]
    fn test_copy_and_clone_pointer() {
        let mut example = Example;
        let pointer = NyahUtilPointer::new(&mut example);
        let pointer_copy = pointer;
        let pointer_clone = pointer.clone();
        assert_eq!(pointer_copy.pointer, pointer_clone.pointer);
    }
}