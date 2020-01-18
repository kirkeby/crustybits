use std::ffi::c_void;
use std::ptr::null;

#[link(name="pcre2-8", kind="dylib")]
extern "C" {
    fn pcre2_compile_8(
        pattern: *const u8, len: usize, opts: u32,
        errno: *mut i32, errpos: *mut usize,
        ctx: *const c_void
    ) -> *const c_void;

    fn pcre2_code_free_8(code: *const c_void);
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Error {
    pub errno: i32,
    pub offset: usize,
}

/// `Code` represents a compiled regular expression.
pub struct Code {
    code: *const c_void,
}

impl Code {
    /// Compile a string into a regular expression.
    pub fn compile(s: &str) -> Result<Self, Error> {
        let mut err = Error::default();
        let code = unsafe {
            pcre2_compile_8(
                s.as_ptr(), s.len(), 0,
                &mut err.errno as *mut i32, &mut err.offset as *mut usize,
                null() as *const c_void
            )
        };
        if code.is_null() {
            return Err(err);
        }
        Ok(Code { code })
    }
}

impl Drop for Code {
    fn drop(&mut self) {
        unsafe { pcre2_code_free_8(self.code); }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Error};

    #[test]
    fn it_works() -> Result<(), Error> {
        let _c = Code::compile("Hello, (.*)!")?;
        Ok(())
    }
}
