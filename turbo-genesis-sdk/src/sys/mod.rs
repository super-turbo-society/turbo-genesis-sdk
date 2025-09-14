//! System Module
//!
//! This module exposes the fundamental building blocks for interacting with the Turbo Genesis runtime:
//!
//! - **Macros**: Powerful code-generation helpers for logging, serialization, and more (`pub use macros::*`).  
//! - **random**: High-quality random number generators and utilities.  
//! - **console**: Console logging and debug output macros and functions.  
//! - **hot**: Hot-reload support for in-development workflows.  
//! - **events**: Custom event emission for browser and non-browser environments.  
//! - **local**: Persistent and volatile local storage APIs.  
//! - **time**: Game-tick counters and real-world timestamp utilities.  
//! - **env**: Environment variable access from the host runtime.
//!
//! Each submodule provides a focused set of functions and types to make it easy to write
//! performant, interactive applications on the Turbo Genesis platform.
pub(crate) mod internal;
pub use macros::*;
pub mod console;
pub mod env;
pub mod events;
pub mod hot;
pub mod local;
mod macros;
pub mod random;
pub mod time;
