extern crate libc;
use libc::size_t;

extern {
    fn serd_strlen(str: *const u8, n_bytes: size_t, flags: u32) -> size_t;
}

fn main() {
    unsafe {
        let sz = serd_strlen((&"hi").as_ptr(), 2, 0);
        println!("hi{}", sz);
    }
}
