use std::ffi::c_void;
use std::ptr::null;

// FIXME - wrap in priavte mod

// FIXME - enums?
type ContextPtr = *const c_void;
type CodePtr = *const c_void;
type MatchDataPtr = *const c_void;
type PcreStr = *const u8;

// FIXME - build.rs to extract from pcre.h?
const PCRE2_ERROR_NOMATCH: i32 = -1;

#[link(name="pcre2-8", kind="dylib")]
extern "C" {
    fn pcre2_compile_8(
        pattern: PcreStr, len: usize, opts: u32,
        errno: *mut i32, errpos: *mut usize,
        ctx: ContextPtr,
    ) -> CodePtr;

    fn pcre2_code_free_8(code: CodePtr);

    fn pcre2_match_data_create_from_pattern_8(
        code: CodePtr, ctx: ContextPtr,
    ) -> MatchDataPtr;

    fn pcre2_match_data_free_8(code: MatchDataPtr);

    fn pcre2_match_8(
        code: CodePtr, s: PcreStr, l: usize, offset: usize,
        opts: u32, data: MatchDataPtr, ctx: ContextPtr,
    ) -> i32;

    fn pcre2_substring_length_bynumber_8(
        data: MatchDataPtr, n: usize, l: *mut usize,
    ) -> i32;

    fn pcre2_substring_copy_bynumber_8(
        data: MatchDataPtr, n: usize, buf: *mut u8, len: *mut usize,
    ) -> i32;
}

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Default, Clone, Copy)]
pub struct Error {
    pub errno: i32,
    pub offset: usize,
}

/// `Code` represents a compiled regular expression.
pub struct Code {
    ptr: CodePtr,
}

// TODO: use &[u8] instead of &str's?
impl Code {
    /// Compile a string into a regular expression.
    pub fn compile(s: &str) -> Result<Self> {
        let mut err = Error::default();
        let ptr = unsafe {
            pcre2_compile_8(
                s.as_ptr(), s.len(), 0,
                &mut err.errno as *mut i32, &mut err.offset as *mut usize,
                null() as *const c_void
            )
        };
        if ptr.is_null() {
            return Err(err);
        }
        Ok(Code { ptr })
    }

    pub fn search(&self, s: &str) -> Result<Option<Match>> {
        let match_data = MatchData::from_pattern(&self);
        let result = unsafe {
            pcre2_match_8(
                self.ptr, s.as_ptr(), s.len(), 0, 0, match_data.ptr, null()
            )
        };
        if result == PCRE2_ERROR_NOMATCH {
            Ok(None)
        } else if result <= 0 {
            Err(Error { errno: result, offset: 0 })
        } else {
            Ok(Some(Match { data: match_data }))
        }
    }
}

impl Drop for Code {
    fn drop(&mut self) {
        unsafe { pcre2_code_free_8(self.ptr); }
    }
}

/// `Match` represents the successful match of a regular expression against a
/// string.
pub struct Match {
    data: MatchData,
}

impl Match {
    pub fn group(&self, n: usize) -> Result<String> {
        self.data.group(n)
    }
}

/// `MatchData` is the PCRE2-internal representation of a regex match.
struct MatchData {
    ptr: MatchDataPtr,
}

impl MatchData {
    fn from_pattern(code: &Code) -> Self {
        let ptr = unsafe {
            pcre2_match_data_create_from_pattern_8(code.ptr, null())
        };
        if ptr.is_null() {
            panic!("pcre2_match_data_create_from_pattern_8 returned NULL");
        }
        MatchData { ptr }
    }

    fn group(&self, n: usize) -> Result<String> {
        let mut len = 0;
        let err = unsafe {
            pcre2_substring_length_bynumber_8(
                self.ptr, n, &mut len as *mut usize,
            )
        };
        wrap_errno(err)?;

        len = len + 1;
        let mut buf = Vec::with_capacity(len);
        let err = unsafe {
            pcre2_substring_copy_bynumber_8(
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
        unsafe { pcre2_match_data_free_8(self.ptr); }
    }
}

/// Returns either Err if errno < 0 else returns Ok(errno).
fn wrap_errno(errno: i32) -> Result<i32> {
    if errno < 0 {
        Err(Error { errno: errno, offset: 0 })
    } else {
        Ok(errno)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Code, Error};

    #[test]
    fn it_works() -> Result<(), Error> {
        let c = Code::compile("Hello, (.*)!")?;
        let m = c.search("Hello, World!")?;
        assert!(m.is_some());
        let m = m.unwrap();
        assert_eq!(m.group(0)?, "Hello, World!");
        assert_eq!(m.group(1)?, "World");
        Ok(())
    }
}
