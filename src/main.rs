extern crate libc;
use libc::size_t;

//#[link(name = "serd")]
extern {
    fn serd_strlen(str: &u8, n_bytes: &size_t, flags: u32) -> size_t;
}

fn main() {
    println!("hi");
}
