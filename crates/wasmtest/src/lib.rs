unsafe extern "C" {
    unsafe fn exec(addr: u32, size: u32);
    unsafe fn put_wood(x: i32, y: i32, z: i32);
}

#[unsafe(no_mangle)]
pub extern "C" fn hello_world() {
    let s = "say Hello, world!";
    unsafe {
        exec(s.as_ptr() as u32, s.len() as u32);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn squre(a: u32) -> u32 {
    a * a
}

// #[unsafe(no_mangle)]
// pub extern "C" fn sphere(x: i32, y: i32, z: i32, r: i32) {
//     for offset_x in -r..r {
//         for offset_y in -r..r {
//             for offset_z in -r..r {
//                 if offset_x * offset_x + offset_y * offset_y + offset_z * offset_z < r * r {
//                     unsafe { put_wood(x + offset_x, y + offset_y, z + offset_z) };
//                 }
//             }
//         }
//     }
// }

#[unsafe(no_mangle)]
pub extern "C" fn lineh(x: i32, y: i32, z: i32, l: i32) {
    for i in x..x + l {
        unsafe {
            put_wood(i, y, z);
        }
    }
}
