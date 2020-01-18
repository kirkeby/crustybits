use std::ffi::c_void;

#[link(name="pcre2-8", kind="dylib")]
extern "C" {
    pub fn pcre2_compile_8(
        pattern: *const u8, len: usize, opts: u32,
        errno: *mut i32, errpos: *mut usize,
        ctx: *const c_void
    ) -> *const c_void;
}

#[cfg(test)]
mod tests {
    use crate::pcre2_compile_8;
    use std::ffi::c_void;
    use std::ptr::null;

    #[test]
    fn it_works() {
        let pattern = "Hello, (.*)!";
        let mut errno: i32 = 0;
        let mut errpos: usize = 0;
        let code = unsafe {
            pcre2_compile_8(
                pattern.as_ptr(), pattern.len(), 0,
                &mut errno as *mut i32, &mut errpos as *mut usize,
                null() as *const c_void
            )
        };
        assert!(!code.is_null());
    }
}
