use std::path::{Path, PathBuf};

/// Extension trait for `PathBuf` to help with common
/// task on directories.
pub trait BaseDirsEx {
    ///  Recursively create a directory and all of its parent components if they are missing.
    ///  If the directory already exists, this function does nothing.
    ///
    ///  # Returns
    ///  The path of the directory that was created.
    ///
    ///  # Errors
    ///  If the directory creation fails, the error is returned.
    ///
    ///  # Example
    ///  ```rust
    ///  use cross_xdg::{BaseDirs, BaseDirsEx};
    ///  use std::path::PathBuf;
    ///  
    ///  let base_dirs = BaseDirs::new().unwrap();
    ///  let my_sub_config_dir = base_dirs.config_home()
    ///     .join("my_sub_dir")
    ///     .create()
    ///     .unwrap();
    /// ```
    fn create(&self) -> Result<PathBuf, std::io::Error>;
}

impl<T: AsRef<Path>> BaseDirsEx for T {
    fn create(&self) -> Result<PathBuf, std::io::Error> {
        std::fs::create_dir_all(self)?;
        Ok(self.as_ref().to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BaseDirs, test_helper::set_var};
    use serial_test::serial;

    #[test]
    #[serial]
    fn create_sub_dir() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let my_sub_config_dir = base_dirs.config_home().join("my_sub_dir").create().unwrap();

        assert!(my_sub_config_dir.exists());
        std::fs::remove_dir_all(my_sub_config_dir).unwrap();
    }

    #[test]
    #[serial]
    fn create_sub_dir_with_prefix() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::with_prefix("prefix").unwrap();

        let my_sub_config_dir = base_dirs.config_home().create().unwrap();

        assert!(my_sub_config_dir.exists());
        std::fs::remove_dir_all(my_sub_config_dir).unwrap();
    }
}
