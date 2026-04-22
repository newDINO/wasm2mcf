unsafe extern "C" {
    unsafe fn exec(addr: u32, size: u32);
}

#[unsafe(no_mangle)]
pub extern "C" fn test() {
    let s = "say Hello, world!";
    unsafe {
        exec(s.as_ptr() as u32, s.len() as u32);
    }
}
