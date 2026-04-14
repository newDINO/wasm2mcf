// unsafe extern "C" {
//     unsafe fn print(start: *const u8, size: u32);
// }

// #[unsafe(no_mangle)]
// pub fn test() {
//     let s = "Hello, world!";
//     unsafe { print(s.as_ptr(), s.len() as u32) };
// }

#[unsafe(no_mangle)]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub fn square(n: i32) -> i32 {
    n * n
}
