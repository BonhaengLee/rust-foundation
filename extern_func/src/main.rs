use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    let rust_str = "I'll be back";
    let null_terminated = CString::new(rust_str).unwrap();
    let len = unsafe { strlen(null_terminated.as_ptr() as *const i8) };
    if len == rust_str.len() {
        println!("The lengths are equal: {}", len);
    } else {
        println!(
            "The lengths are not equal: strlen returned {}, rust_str.len() returned {}",
            len,
            rust_str.len()
        );
    }
}
