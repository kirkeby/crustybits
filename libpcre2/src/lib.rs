use std::ptr::null;

/// `Code` represents a compiled regular expression.
pub struct Code {
    ptr: ffi::RePtr,
}

// TODO: use &[u8] instead of &str's?
impl Code {
    /// Compile a string into a regular expression.
    pub fn compile(s: &str) -> Result<Self> {
        let mut errno = 0;
        let mut offset = 0;
        let ptr = unsafe {
            ffi::pcre2_compile_8(
                s.as_ptr(), s.len(), 0,
                &mut errno as *mut i32, &mut offset as *mut usize,
                null(),
            )
        };
        if ptr.is_null() {
            return Err(Error::CompileError(errno, offset));
        }
        Ok(Code { ptr })
    }

    /// Find first match of regular expression in string `s`.
    pub fn search(&self, s: &str) -> Option<Match> {
        let match_data = MatchData::from_pattern(&self);
        let result = unsafe {
            ffi::pcre2_match_8(
                self.ptr, s.as_ptr(), s.len(), 0, 0, match_data.ptr, null(),
            )
        };
        if result == ffi::PCRE2_ERROR_NOMATCH {
            None
        } else if result <= 0 {
            panic!("BUG? error {:?} from pcre2_match_8", result);
        } else {
            Some(Match { data: match_data, matches: result as usize })
        }
    }
}

impl Drop for Code {
    fn drop(&mut self) {
        unsafe { ffi::pcre2_code_free_8(self.ptr); }
    }
}

/// `Match` represents the successful match of a regular expression against a
/// string.
pub struct Match {
    data: MatchData,
    matches: usize,
}

impl Match {
    pub fn group(&self, n: usize) -> Result<String> {
        self.data.group(n)
    }

    pub fn groups(&self) -> Result<Vec<String>> {
        (1..self.matches).map(|n| self.group(n)).collect()
    }
}

/// `MatchData` is the PCRE2-internal representation of a regex match.
struct MatchData {
    ptr: ffi::MatchDataPtr,
}

impl MatchData {
    fn from_pattern(code: &Code) -> Self {
        let ptr = unsafe {
            ffi::pcre2_match_data_create_from_pattern_8(code.ptr, null())
        };
        if ptr.is_null() {
            panic!("pcre2_match_data_create_from_pattern_8 returned NULL");
        }
        MatchData { ptr }
    }

    fn group(&self, n: usize) -> Result<String> {
        let mut len = 0;
        let err = unsafe {
            ffi::pcre2_substring_length_bynumber_8(
                self.ptr, n, &mut len as *mut usize,
            )
        };
        wrap_errno(err)?;

        len = len + 1;
        let mut buf = Vec::with_capacity(len);
        let err = unsafe {
            ffi::pcre2_substring_copy_bynumber_8(
                self.ptr, n, buf.as_mut_ptr(), &mut len as *mut usize,
            )
        };
        wrap_errno(err)?;

        // This is okay because pcre_substring_copy_bynumber_8 has initalized
        // len + 1 bytes in buf.
        unsafe { buf.set_len(len); }
        Ok(String::from_utf8(buf).unwrap())
    }
}

impl Drop for MatchData {
    fn drop(&mut self) {
        unsafe { ffi::pcre2_match_data_free_8(self.ptr); }
    }
}


/// Returns either Err if errno < 0 else returns Ok(errno).
// FIXME - replace with macro and move into ffi?
fn wrap_errno(errno: i32) -> Result<i32> {
    if errno < 0 {
        Err(Error::FfiError(errno))
    } else {
        Ok(errno)
    }
}


pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Error compiling regular expression, values are errno and offset
    /// into regular expression.
    CompileError(i32, usize),

    /// All other errors from the C API.
    FfiError(i32),
}

/// Internal module with C API definitions.
mod ffi {
    use std::ffi::c_void;

    // FIXME - enums?
    pub type ContextPtr = *const c_void;
    pub type RePtr = *const c_void;
    pub type MatchDataPtr = *const c_void;
    pub type PcreStr = *const u8;

    // FIXME - build.rs to extract from pcre.h?
    pub const PCRE2_ERROR_NOMATCH: i32 = -1;

    #[link(name="pcre2-8", kind="dylib")]
    extern "C" {
        pub fn pcre2_compile_8(
            pattern: PcreStr, len: usize, opts: u32,
            errno: *mut i32, errpos: *mut usize,
            ctx: ContextPtr,
        ) -> RePtr;

        pub fn pcre2_code_free_8(code: RePtr);

        pub fn pcre2_match_data_create_from_pattern_8(
            code: RePtr, ctx: ContextPtr,
        ) -> MatchDataPtr;

        pub fn pcre2_match_data_free_8(code: MatchDataPtr);

        pub fn pcre2_match_8(
            code: RePtr, s: PcreStr, l: usize, offset: usize,
            opts: u32, data: MatchDataPtr, ctx: ContextPtr,
        ) -> i32;

        pub fn pcre2_substring_length_bynumber_8(
            data: MatchDataPtr, n: usize, l: *mut usize,
        ) -> i32;

        pub fn pcre2_substring_copy_bynumber_8(
            data: MatchDataPtr, n: usize, buf: *mut u8, len: *mut usize,
        ) -> i32;
    }
}


#[cfg(test)]
mod tests {
    use crate::{Code, Error};

    #[test]
    fn it_works() -> Result<(), Error> {
        let c = Code::compile("Hello, (.*)!")?;
        let m = c.search("Hello, World!");
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.group(0)?, "Hello, World!");
        assert_eq!(m.group(1)?, "World");
        assert_eq!(m.groups()?, vec!["World"]);
        Ok(())
    }
}
