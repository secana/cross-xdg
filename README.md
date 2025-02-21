# cross-xdg

    THIS CRATE IS CURRENTLY WORK IN PROGRESS
    There will be frequent breaking changes until the first 1.*.* release

Use the [XDG base directory specification](https://specifications.freedesktop.org/basedir-spec/latest/) on Linux, Windows and Mac.

The XDG base directory specification is a standard for storing user-specific configuration, data, cache and runtime files. It is used by many Linux applications and desktop environments, but is not widely supported on other platforms.

This library provides a cross-platform implementation of the XDG base directory specification. It works on Linux, Windows and Mac, and provides a simple API for accessing the standard directories.

## Details

For example, lets take the `XDG_CONFIG_HOME` directory. The `XDG_CONFIG_HOME` directory is used to store user-specific configuration files. On Linux, this is usually `~/.config`, but it can be overridden by setting the `XDG_CONFIG_HOME` environment variable. This crate will either return the path from `XDG_CONFIG_HOME` or the default `home` directory for the corresponding platform.

- On Linux: `/home/<user>/.config` or `$XDG_CONFIG_HOME`
- On Windows `C:\Users\<user>\.config` or `$XDG_CONFIG_HOME`
- On Mac: `/Users/<user>/.config` or `$XDG_CONFIG_HOME`


All the other XDG directories are implemented in the same way in this crate.

- `XDG_DATA_HOME` - User-specific data files
- `XDG_CACHE_HOME` - User-specific non-essential data files
- `XDG_STATE_HOME` - User-specific application state files
- `XDG_CONFIG_HOME` - User-specific configuration files
- `XDG_RUNTIME_DIR` - User-specific runtime files

## Releated Projects

There are several related projects that provide similar functionality, but do not conform to the XDG base directory specification. Instead they use platform-specific conventions.

- [directories](https://crates.io/crates/directories)
- [directories-next](https://crates.io/crates/directories-next)
- [directories-rs](https://crates.io/crates/directories-rs)

XDG conformant, but not cross-platform:

- [xdg](https://crates.io/crates/xdg)
