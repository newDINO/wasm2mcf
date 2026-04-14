unsafe extern "C" {
    unsafe fn exec(start: *const u8, size: u32);
}

#[unsafe(no_mangle)]
pub fn test() {
    let s = "say 'Hello, world!'";
    unsafe { exec(s.as_ptr(), s.len() as u32) };
}

// #[unsafe(no_mangle)]
// pub fn add(a: i32, b: i32) -> i32 {
//     a + b
// }

// #[unsafe(no_mangle)]
// pub fn square(n: i32) -> i32 {
//     n * n
// }
