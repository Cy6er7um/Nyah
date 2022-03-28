#[macro_export]
macro_rules! call_extern_code {
    ($func:expr) => {
        $func
    };
    ($pointer:expr) => {
        unsafe {
            use std::mem::transmute;
            let function = transmute::<
                _,
                extern fn() -> _
            >($pointer);
            function()
        }
    };
    ($pointer:expr,) => {
        $crate::call_extern_code!($pointer)
    };
    ($pointer:expr, $($arg:expr),*) => {
        unsafe {
            use std::mem::transmute;
            let function = transmute::<
                _,
                extern fn($($crate::to_underscore!($arg)),*) -> _
            >($pointer);
            function($($arg),*)
        }
    };
    ($pointer:expr, $($arg:expr),*,) => {
        $crate::call_extern_code!($pointer, $($arg),*)
    };
}

#[cfg(test)]
mod test {
    extern fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    #[test]
    fn test_call_extern_code() {
        let pointer = add as *const u8;
        let result: i32 = call_extern_code!(pointer, 1, 2,);
        assert_eq!(result, 3);
    }
}
