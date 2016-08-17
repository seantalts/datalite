extern crate libc;
use libc::size_t;
use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn serd_strlen(str: *const c_char, n_bytes: &size_t, flags: *const u32) -> size_t;
}

fn main() {
    unsafe {
        let s = CString::new("hello").unwrap();
        let flags: u32 = 0;
        let size: size_t = 5;
        let sz = serd_strlen(s.as_ptr(), &size, &flags);
        println!("hi{}", sz);
    }
}
