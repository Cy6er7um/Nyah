use std::fmt::{Debug, Display, Pointer};

use libffi::high::{Arg, call, CodePtr, CType};

use crate::util::any_pointer::NyahUtilAnyPointer;
use crate::util::pointer::NyahUtilPointer;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NyahFunctionPointer {
    pub pointer: NyahUtilAnyPointer,
}

impl NyahFunctionPointer {
    pub const fn new(pointer: NyahUtilAnyPointer) -> Self {
        Self {
            pointer
        }
    }

    pub fn cast_function<T: Copy>(&self) -> T {
        // If use `transmute` here, the compiler will throw a error.
        let pointer = self as *const _ as *const T;
        unsafe {
            *pointer
        }
    }

    pub fn call_ffi_high<R: CType>(&self, args: &[Arg]) -> R {
        unsafe {
            call::<R>(
                CodePtr::from_ptr(
                    self.pointer.pointer.cast()
                ),
                args,
            )
        }
    }
}

impl<T> From<NyahUtilPointer<T>> for NyahFunctionPointer {
    fn from(pointer: NyahUtilPointer<T>) -> Self {
        Self::new(
            pointer.cast(),
        )
    }
}

impl<T> From<*mut T> for NyahFunctionPointer {
    fn from(pointer: *mut T) -> Self {
        NyahUtilPointer::new(
            pointer
        ).into()
    }
}

impl<T> From<*const T> for NyahFunctionPointer {
    fn from(pointer: *const T) -> Self {
        (pointer as *mut T).into()
    }
}

impl Display for NyahFunctionPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<FunctionPointer at {:p}>", self.pointer)
    }
}

impl Debug for NyahFunctionPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Pointer for NyahFunctionPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Pointer::fmt(&self.pointer, f)
    }
}

#[cfg(test)]
mod test {
    use crate::function::pointer::NyahFunctionPointer;

    extern fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn test_cast_function() {
        let function_pointer = NyahFunctionPointer::from(
            add as *const u8
        );
        let result = function_pointer
            .cast_function::<extern fn(i32, i32) -> i32>()(1, 2);
        assert_eq!(result, 3);
    }
}
