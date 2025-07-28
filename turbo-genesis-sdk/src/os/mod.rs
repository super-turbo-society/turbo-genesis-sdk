//! Turbo OS Module
//!
//! This module brings together the key macros and submodules for Turbo OS,
//! and provides the `HasProgramId` trait for types that define a program ID.

use super::*;

/// Re-export of Turbo OS procedural macros for channel, command, document, and program.
pub use turbo_genesis_macros::{channel, command, document};

/// Client-side FFI bindings and convenience wrappers.
pub mod client;

/// Server-side FFI bindings and utilities.
pub mod server;

/// Trait for types that have an associated Turbo OS program ID.
///
/// # Associated Constants
/// - `PROGRAM_ID`: A static string slice uniquely identifying the program.
pub trait HasProgramId {
    /// A Turbo OS program ID used for file paths and command routing.
    const PROGRAM_ID: &'static str;
}
