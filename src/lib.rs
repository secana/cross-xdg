//! Cross plattform XDG base directory specification.
//!
//! This library provides a way to access the XDG base directory specification on all platforms.
//! The XDG base directory specification is a standard for storing user-specific configuration,
//! data, cache, and runtime files. It is used by many Linux desktop environments and applications.
//! The specification is defined in the [XDG Base Directory
//! Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).
//!
//! In contrast to other XDG base directory crates or standard directory crates, `cross-xdg`
//! provides XDG directories on Windows and macOS just like on Linux.
//!
//! Example for Linux:
//! ```rust
//! use cross_xdg::BaseDirs;
//! let base_dirs = BaseDirs::new().unwrap();
//!
//! // On Linux: resolves to /home/<user>/.config
//! // On Windows: resolves to C:\Users\<user>\.config
//! // On macOS: resolves to /Users/<user>/.config
//! let config_home = base_dirs.config_home();
//! ```

mod base_dirs;
mod base_dirs_ex;
mod test_helper;

pub use base_dirs::BaseDirs;
pub use base_dirs_ex::BaseDirsEx;
