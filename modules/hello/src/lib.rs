#![no_std]

use kmod::{declare_module, exit_fn, init_fn};

unsafe extern "C" {
    fn write_char(c: u8);
}

struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            unsafe { write_char(b) };
        }
        Ok(())
    }
}

#[init_fn]
pub fn hello_init() -> i32 {
    let mut writer = Writer;
    core::fmt::write(&mut writer, format_args!("Hello, Kernel Module!\n")).unwrap();
    0
}

#[exit_fn]
fn hello_exit() {
    let mut writer = Writer;
    core::fmt::write(&mut writer, format_args!("Goodbye, Kernel Module!\n")).unwrap();
}

declare_module!("hello", "1.0.0", hello_init, hello_exit);
