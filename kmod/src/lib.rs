#![no_std]
#![feature(linkage)]

use core::fmt::Debug;
pub use kmacro::{exit_fn, init_fn};

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ModuleInfo {
    pub magic: u32,
    pub name: [u8; 64],
    pub version: [u8; 32],
    pub init_fn: Option<fn() -> i32>,
    pub exit_fn: Option<fn()>,
}

impl Default for ModuleInfo {
    fn default() -> Self {
        ModuleInfo {
            magic: 0,
            name: [0; 64],
            version: [0; 32],
            init_fn: None,
            exit_fn: None,
        }
    }
}

impl Debug for ModuleInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ModuleInfo {{ name: {}, version: {}, init_fn: {:?}, exit_fn: {:?} }}",
            self.name(),
            self.version(),
            self.init_fn,
            self.exit_fn,
        )
    }
}

impl ModuleInfo {
    pub fn name(&self) -> &str {
        let len = self
            .name
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..len]).unwrap_or("Invalid UTF-8")
    }

    pub fn version(&self) -> &str {
        let len = self
            .version
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.version.len());
        core::str::from_utf8(&self.version[..len]).unwrap_or("Invalid UTF-8")
    }
}

// "MODU"
pub const MODULE_MAGIC: u32 = 0x4D4F4455;

#[macro_export]
macro_rules! declare_module {
    ($name:expr, $version:expr, $init:expr, $exit:expr) => {
        #[used]
        #[link_section = ".modinfo"]
        pub static MODULE_INFO: $crate::ModuleInfo = $crate::ModuleInfo {
            magic: $crate::MODULE_MAGIC,
            name: $crate::str_to_array64($name),
            version: $crate::str_to_array32($version),
            init_fn: Some($init),
            exit_fn: Some($exit),
        };

        #[cfg(target_os = "none")]
        #[panic_handler]
        fn panic(_info: &core::panic::PanicInfo) -> ! {
            loop {}
        }
    };
}

pub const fn str_to_array64(s: &str) -> [u8; 64] {
    let mut array = [0u8; 64];
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && i < 63 {
        array[i] = bytes[i];
        i += 1;
    }
    array
}

pub const fn str_to_array32(s: &str) -> [u8; 32] {
    let mut array = [0u8; 32];
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() && i < 31 {
        array[i] = bytes[i];
        i += 1;
    }
    array
}
