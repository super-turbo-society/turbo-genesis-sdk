#![allow(unused, static_mut_refs, unexpected_cfgs)]

pub(crate) mod ffi;

pub mod audio;
pub mod bounds;
pub mod canvas;
pub mod http;
#[allow(deprecated)]
pub mod input;
pub mod os;
pub mod sys;
pub mod tween;

#[cfg(feature = "solana")]
pub mod solana;

pub use borsh;
pub use structstruck;
pub use toml;

#[allow(unused_imports)]
pub mod prelude {
    pub use crate::bounds::*;
    #[allow(unused_imports)]
    pub use crate::bounds::*;
    pub use crate::canvas::*;
    pub use crate::input::*;
    pub use crate::println;
    pub use crate::sys::*;
    pub use crate::tween::*;
    pub use crate::*;
}

#[macro_export]
macro_rules! println {
    ($fmt:expr $(, $($arg:tt)*)?) => {
        $crate::sys::log(&format!($fmt, $($($arg)*)?))
    };
}

#[macro_export]
macro_rules! config {
    ($($data:tt)*) => {
        #[no_mangle]
        pub unsafe extern "C" fn config() -> u64 {
            use $crate::toml;
            let t = toml::toml!($($data)*).to_string();
            let ptr = t.as_ptr() as u64;
            let len = t.len() as u64;
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

        #[cfg(not(turbo_hot_reload))]
        static mut GAME_STATE: Option<$StructName> = None;

        #[cfg(not(turbo_hot_reload))]
        impl $StructName {
            pub fn default() -> Self {
                $default
            }
            pub fn load() -> Self {
                unsafe { GAME_STATE.take().unwrap_or_else(Self::default) }
            }
            pub fn save(self) -> bool {
                unsafe { GAME_STATE = Some(self) };
                true
            }
        }

        #[cfg(turbo_hot_reload)]
        impl $StructName {
            pub fn default() -> Self {
                $default
            }
            pub fn load() -> Self {
                match $crate::sys::load() {
                    Ok(bytes) => $StructName::try_from_slice(&bytes).unwrap_or_else(|_| Self::default()),
                    Err(_) => Self::default()
                }
            }
            pub fn save(self) -> bool {
                if let Ok(bytes) = $StructName::try_to_vec(&self) {
                    if let Ok(_) = $crate::sys::save(&bytes) {
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

        #[cfg(not(turbo_no_run))]
        #[no_mangle]
        #[allow(overflowing_literals, non_upper_case_globals)]
        pub unsafe extern "C" fn run() {
            use std::f32::consts::PI;
            $($body)*
        }

        #[cfg(turbo_no_run)]
        #[allow(overflowing_literals, non_upper_case_globals)]
        unsafe fn run() {
            use std::f32::consts::PI;
            $($body)*
        }
    };
}
