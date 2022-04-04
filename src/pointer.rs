use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

use crate::untyped::Untyped;

pub struct Pointer<T = Untyped> {
    pub pointer: *mut T,
}

impl<T> Pointer<T> {
    pub const fn new(pointer: *mut T) -> Self {
        Self {
            pointer,
        }
    }

    pub const fn _cast<U>(&self) -> Pointer<U> {
        Pointer::new(self.pointer as *mut U)
    }

    pub unsafe fn _offset(self, count: isize) -> Self {
        Self {
            pointer: self.pointer.offset(count),
        }
    }

    pub unsafe fn _add(self, count: usize) -> Self {
        Self {
            pointer: self.pointer.add(count),
        }
    }

    pub unsafe fn _sub(self, count: usize) -> Self {
        Self {
            pointer: self.pointer.sub(count),
        }
    }

    pub unsafe fn _write(&self, value: T) {
        self.pointer.write(value);
    }

    pub unsafe fn _read(&self) -> T {
        self.pointer.read()
    }

    pub fn _as_ref(&self) -> Option<&T> {
        unsafe {
            self.pointer.as_ref()
        }
    }

    pub unsafe fn _as_ref_unchecked(&self) -> &T {
        &*self.pointer
    }

    pub fn _as_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.pointer.as_mut()
        }
    }

    pub unsafe fn _as_mut_unchecked(&self) -> &mut T {
        &mut *self.pointer
    }
}

impl<T> Clone for Pointer<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Pointer<T> {}

impl<T, O> PartialEq<Pointer<O>> for Pointer<T> {
    fn eq(&self, other: &Pointer<O>) -> bool {
        self.pointer.eq(
            &other.pointer.cast()
        )
    }
}

impl<T> Eq for Pointer<T> {}

impl<T, O> PartialOrd<Pointer<O>> for Pointer<T> {
    fn partial_cmp(&self, other: &Pointer<O>) -> Option<Ordering> {
        self.pointer.partial_cmp(
            &other.pointer.cast()
        )
    }
}

impl<T> Ord for Pointer<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pointer.cmp(
            &other.pointer.cast()
        )
    }
}

impl<T> Deref for Pointer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self._as_ref()
            .expect("The null pointer cannot be dereferenced.")
    }
}

impl<T> DerefMut for Pointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self._as_mut()
            .expect("The null pointer cannot be dereferenced.")
    }
}

impl<T> AsRef<T> for Pointer<T> {
    fn as_ref(&self) -> &T {
        self.deref()
    }
}

impl<T> AsMut<T> for Pointer<T> {
    fn as_mut(&mut self) -> &mut T {
        self.deref_mut()
    }
}

impl<T> From<*mut T> for Pointer<T> {
    fn from(ptr: *mut T) -> Self {
        Self::new(ptr)
    }
}

impl<T> Into<*mut T> for Pointer<T> {
    fn into(self) -> *mut T {
        self.pointer
    }
}

impl<T: Display> Display for Pointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.as_ref(), f)
    }
}

impl<T: Debug> Debug for Pointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.as_ref(), f)
    }
}

impl<T> std::fmt::Pointer for Pointer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Pointer::fmt(&self.pointer, f)
    }
}

impl<T> Hash for Pointer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pointer.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use crate::pointer::Pointer;

    #[test]
    fn pointer_cast() {
        let pointer = Pointer::new(0x1111 as *mut u8);
        let pointer_cast = pointer._cast::<u16>();
        assert_eq!(pointer_cast.pointer as usize, 0x1111);
    }

    #[test]
    fn pointer_offset_add_sub() {
        let pointer = Pointer::new(0x1111 as *mut u8);
        let pointer_offset = unsafe { pointer._offset(1) };
        let pointer_add = unsafe { pointer._add(1) };
        let pointer_sub = unsafe { pointer._sub(1) };
        assert_eq!(pointer_offset.pointer as usize, 0x1112);
        assert_eq!(pointer_add.pointer as usize, 0x1112);
        assert_eq!(pointer_sub.pointer as usize, 0x1110);
    }

    #[test]
    fn pointer_format() {
        let mut string = String::from("Hello, world!");
        let pointer = Pointer::new(&mut string);

        let format_result = format!("{}", pointer);
        assert_eq!(format_result, "Hello, world!");

        let format_result = format!("{:?}", pointer);
        assert_eq!(format_result, "\"Hello, world!\"");

        let format_result = format!("{:p}", pointer);
        let pointer_address = format!("{:p}", pointer.pointer);
        assert_eq!(format_result, pointer_address.as_str());
    }

    #[test]
    fn pointer_read_write() {
        let mut integer = 1111;

        {
            let mut pointer = Pointer::new(&mut integer);
            *pointer = 2222;
        }
        assert_eq!(integer, 2222);

        unsafe {
            let pointer = Pointer::new(&mut integer);
            pointer._write(3333);
        }
        assert_eq!(integer, 3333);
    }
}