use std::ffi::{c_int, c_uchar, c_uint};

unsafe extern "C" {
    unsafe fn init_float_window() -> c_int;
    unsafe fn toast(msg: *const u16, time: c_uint);
    unsafe fn setPos(x: c_int, y: c_int);
    unsafe fn setWH(width: c_int, height: c_int);
    unsafe fn setAlpha(alpha: c_uchar);
}

#[cfg(windows)]
pub fn show_toast(msg: &str, time: u32) {
    use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

    let wide: Vec<u16> = OsStr::new(msg)
        .encode_wide() // 将 UTF-8 转为 UTF-16 代码单元迭代器
        .chain(Some(0)) // 添加 nul 终止符（C 宽字符串需要）
        .collect();
    unsafe {
        init_float_window();
        toast(wide.as_ptr(), time);
    }
}

#[cfg(windows)]
pub fn set_toast_position(x: c_int, y: c_int) {
    unsafe {
        setPos(x, y);
    }
}

#[cfg(windows)]
pub fn set_toast_wh(width: c_int, height: c_int) {
    unsafe {
        setWH(width, height);
    }
}

#[cfg(windows)]
pub fn set_toast_alpha(alpha: u8) {
    unsafe {
        setAlpha(alpha);
    }
}
