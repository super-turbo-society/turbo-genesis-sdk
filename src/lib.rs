pub(crate) mod ffi;

pub mod canvas;
pub mod input;
pub mod sys;

#[cfg(feature = "solana")]
pub mod solana;

pub use binary_layout;
pub use borsh;
pub use paste;
pub use structstruck;

pub mod prelude {
    pub use crate::canvas::*;
    pub use crate::input::*;
    pub use crate::println;
    pub use crate::sys::*;
    pub use crate::*;
}

pub fn run_snapshot(snapshot_data: &[u8], run: impl FnOnce()) -> Vec<u8> {
    ffi::internal::write_snapshot(snapshot_data);
    run();
    ffi::internal::read_snapshot_state()
}

#[macro_export]
macro_rules! println {
    ($fmt:expr $(, $($arg:tt)*)?) => { $crate::sys::log(&format!($fmt, $($($arg)*)?)) };
}

#[macro_export]
macro_rules! cfg {
    ($toml:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn config() -> u64 {
            let ptr = $toml.as_ptr() as u64;
            let len = $toml.len() as u64;
            (len << 32 | ptr)
        }
    };
}

#[macro_export]
macro_rules! init {
    (struct $StructName:ident { $($fields:tt)* } = $default:expr) => {
        use $crate::prelude::{*, println};
        use $crate::borsh::{self, *};
        use $crate::structstruck::{self, *};
        strike! {
            #[strikethrough[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]]
            struct $StructName {
                $($fields)*
            }
        }
        impl $StructName {
            pub fn load() -> Self {
                let state = $crate::sys::load()
                    .and_then(|xs| $StructName::try_from_slice(&xs).map_err(|err| -1))
                    .unwrap_or_else(|_| $default);
                std::println!("Loaded {:?}", state);
                state
            }
            pub fn save(&self) -> bool {
                if let Ok(bytes) = $StructName::try_to_vec(&self) {
                    if let Ok(_) = $crate::sys::save(&bytes) {
                        std::println!("Saved {:?}", self);
                        return true;
                    }
                }
                return false;
            }
        }
    };
}

#[macro_export]
macro_rules! go {
    ($($body:tt)*) => {
        use $crate::prelude::*;

        #[no_mangle]
        #[allow(overflowing_literals, non_upper_case_globals)]
        pub unsafe extern "C" fn run() {
            use std::f32::consts::PI;
            $($body)*
        }
        pub fn run_snapshot(snapshot_data: &[u8]) -> Vec<u8> {
            $crate::run_snapshot(snapshot_data, || unsafe { run() })
        }
    };
}
