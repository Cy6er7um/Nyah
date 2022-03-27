use std::fmt::{Debug, Display, Pointer};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::ptr;

#[repr(C)]
pub struct NyahUtilPointer<T> {
    pub pointer: *mut T,
}

impl<T> NyahUtilPointer<T> {
    pub const NULL: Self = Self { pointer: ptr::null_mut() };

    pub const fn new(pointer: *mut T) -> Self {
        Self {
            pointer,
        }
    }

    pub fn cast<U>(&self) -> Self {
        Self {
            pointer: self.pointer.cast(),
        }
    }

    pub fn is_null(&self) -> bool {
        self.pointer.is_null()
    }

    pub fn is_not_null(&self) -> bool {
        !self.is_null()
    }

    pub fn as_ptr(&self) -> *mut T {
        self.pointer
    }

    pub fn offset(&self, offset: isize) -> Self {
        Self {
            pointer: unsafe {
                self.pointer.offset(offset)
            },
        }
    }

    pub fn add(&self, offset: usize) -> Self {
        Self {
            pointer: unsafe {
                self.pointer.add(offset)
            },
        }
    }

    pub fn sub(&self, offset: usize) -> Self {
        Self {
            pointer: unsafe {
                self.pointer.sub(offset)
            },
        }
    }

    pub fn offset_from(&self, other: &Self) -> isize {
        unsafe {
            self.pointer.offset_from(other.pointer)
        }
    }
}

impl<T> Clone for NyahUtilPointer<T> {
    fn clone(&self) -> Self {
        Self {
            // Copy the pointer here.
            pointer: self.pointer,
        }
    }
}

// If you use derive to automatically implement Copy,
// then Pointer will not be able to Copy when T does not implement Copy.
impl<T> Copy for NyahUtilPointer<T> {}


impl<T> From<*mut T> for NyahUtilPointer<T> {
    fn from(ptr: *mut T) -> Self {
        Self::new(ptr)
    }
}

impl<T> Into<*mut T> for NyahUtilPointer<T> {
    fn into(self) -> *mut T {
        self.pointer
    }
}

impl<T> AsRef<T> for NyahUtilPointer<T> {
    fn as_ref(&self) -> &T {
        unsafe {
            self.pointer.as_ref()
                .expect("A null pointer cannot be converted to a reference.")
        }
    }
}

impl<T> AsMut<T> for NyahUtilPointer<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe {
            self.pointer.as_mut()
                .expect("A null pointer cannot be converted to a reference.")
        }
    }
}

impl<T> Deref for NyahUtilPointer<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T> DerefMut for NyahUtilPointer<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.as_mut()
    }
}

impl<LT, RT> PartialEq<NyahUtilPointer<RT>> for NyahUtilPointer<LT> {
    fn eq(&self, other: &NyahUtilPointer<RT>) -> bool {
        self.pointer == other.pointer.cast()
    }
}

impl<T> Eq for NyahUtilPointer<T> {}

impl<T> Pointer for NyahUtilPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:p}", self.pointer)
    }
}

impl<T: Debug> Debug for NyahUtilPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<T: Display> Display for NyahUtilPointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl<T> Hash for NyahUtilPointer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pointer.hash(state)
    }
}

#[cfg(test)]
mod test {
    use crate::util::pointer::NyahUtilPointer;

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