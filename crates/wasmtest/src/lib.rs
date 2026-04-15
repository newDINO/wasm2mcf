unsafe extern "C" {
    unsafe fn exec(start: *const u8, size: u32);
}

#[unsafe(no_mangle)]
pub fn test() {
    let x = 0;
    let y = 16;
    let z = 0;
    let s = format!("tp {} {} {}", x, y, z);
    unsafe { exec(s.as_ptr(), s.len() as u32) };
}
