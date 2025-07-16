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
    ///  ```no_run
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

    /// Copy the content of a file to the directory.
    /// If the source is a file, it will be copied to the directory.
    /// If the file already exists, it will be overwritten.
    /// The destination directory will be created if it does not exist.
    ///
    /// # Parameters
    /// - `name`: The name of the file to create.
    /// - `src`: The source file to copy.
    ///
    /// # Returns
    /// The path of the file where the source was copied.
    ///
    /// # Examles
    /// ```no_run
    /// use cross_xdg::{BaseDirs, BaseDirsEx};
    /// use std::path::PathBuf;
    ///
    /// let base_dirs = BaseDirs::new().unwrap();
    ///
    /// let src = PathBuf::from("/tmp/src.txt");
    /// let my_sub_config_dir = base_dirs.config_home()
    ///    .join("my_sub_dir")
    ///    .copy_over("dest.txt", src)
    ///    .unwrap();
    /// ```
    fn copy_over(&self, name: &str, src: impl AsRef<Path>) -> Result<PathBuf, std::io::Error>;

    /// Copy the content of a file to the directory.
    /// If the source is a file, it will be copied to the directory.
    /// If the file already exists, it will *not* be overwritten.
    /// The destination directory will be created if it does not exist.
    /// The file will be created with the name specified in the `name` parameter.
    ///
    /// # Parameters
    /// - `name`: The name of the file to create.
    /// - `src`: The source file to copy.
    ///
    /// # Returns
    /// The path of the file where the source was copied.
    ///
    /// # Examles
    /// ```no_run
    /// use cross_xdg::{BaseDirs, BaseDirsEx};
    /// use std::path::PathBuf;
    ///
    /// let base_dirs = BaseDirs::new().unwrap();
    /// let src = PathBuf::from("/tmp/src.txt");
    /// let my_sub_config_dir = base_dirs.config_home()
    ///   .join("my_sub_dir")
    ///   .copy("dest.txt", src)
    ///   .unwrap();
    /// ```
    fn copy(&self, name: &str, src: impl AsRef<Path>) -> Result<PathBuf, std::io::Error>;

    /// Write the content of a byte slice to a file in the directory.
    /// The destination directory will be created if it does not exist.
    /// If the file already exists, it will be overwritten.
    /// The file will be created with the name specified in the `name` parameter.
    ///
    /// # Parameters
    /// - `name`: The name of the file to create.
    /// - `src`: The content to write to the file.
    ///
    /// # Returns
    /// The path of the file that was created.
    ///
    /// # Examples
    /// ```no_run
    /// use cross_xdg::{BaseDirs, BaseDirsEx};
    /// use std::path::PathBuf;
    ///
    /// let base_dirs = BaseDirs::new().unwrap();
    /// let my_file = base_dirs.config_home()
    ///   .write_over("hello.txt", b"Hello, World!")
    ///   .unwrap();
    /// ```
    fn write_over(&self, name: &str, src: &[u8]) -> Result<PathBuf, std::io::Error>;

    /// Write the content of a byte slice to a file in the directory.
    /// The destination directory will be created if it does not exist.
    /// If the file already exists, it will not be overwritten.
    /// The file will be created with the name specified in the `name` parameter.
    ///
    /// # Parameters
    /// - `name`: The name of the file to create.
    /// - `src`: The content to write to the file.
    ///
    /// # Returns
    /// The path of the file that was created.
    ///
    /// # Examples
    /// ```no_run
    /// use cross_xdg::{BaseDirs, BaseDirsEx};
    /// use std::path::PathBuf;
    ///
    /// let base_dirs = BaseDirs::new().unwrap();
    /// let my_file = base_dirs.config_home()
    ///   .write("hello.txt", b"Hello, World!")
    ///   .unwrap();
    /// ```
    fn write(&self, name: &str, src: &[u8]) -> Result<PathBuf, std::io::Error>;
}

impl<T: AsRef<Path>> BaseDirsEx for T {
    fn create(&self) -> Result<PathBuf, std::io::Error> {
        std::fs::create_dir_all(self)?;
        Ok(self.as_ref().to_path_buf())
    }

    fn copy_over(&self, name: &str, src: impl AsRef<Path>) -> Result<PathBuf, std::io::Error> {
        let dir = self.create()?;
        let full_path = dir.join(name);
        std::fs::copy(&src, &full_path)?;
        Ok(dir.join(full_path))
    }

    fn write_over(&self, name: &str, src: &[u8]) -> Result<PathBuf, std::io::Error> {
        let dir = self.create()?;
        let full_path = dir.join(name);
        std::fs::write(&full_path, src)?;
        Ok(full_path)
    }

    fn copy(&self, name: &str, src: impl AsRef<Path>) -> Result<PathBuf, std::io::Error> {
        let dest = self.as_ref().join(name);
        if dest.exists() {
            Ok(dest)
        } else {
            self.copy_over(name, src)
        }
    }

    fn write(&self, name: &str, src: &[u8]) -> Result<PathBuf, std::io::Error> {
        let dest = self.as_ref().join(name);
        if dest.exists() {
            Ok(dest)
        } else {
            self.write_over(name, src)
        }
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

        assert!(my_sub_config_dir.starts_with("/tmp/config/my_sub_dir"));
        std::fs::remove_dir_all(my_sub_config_dir).unwrap();
    }

    #[test]
    #[serial]
    fn create_sub_dir_with_prefix() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::with_prefix("prefix").unwrap();

        let my_sub_config_dir = base_dirs.config_home().create().unwrap();

        assert!(my_sub_config_dir.starts_with("/tmp/config/prefix"));
        std::fs::remove_dir_all(my_sub_config_dir).unwrap();
    }

    #[test]
    #[serial]
    fn copy_over_file() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let src = PathBuf::from("/tmp/src.txt");
        std::fs::write(&src, b"Hello, World!").unwrap();

        let my_sub_config_dir = base_dirs
            .config_home()
            .join("my_sub_dir")
            .copy_over("hello.txt", &src)
            .unwrap();

        assert!(my_sub_config_dir.starts_with("/tmp/config/my_sub_dir/hello.txt"));
        std::fs::remove_dir_all(my_sub_config_dir.parent().unwrap()).unwrap();
    }

    #[test]
    #[serial]
    fn copy_file_not_exists() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let src = PathBuf::from("/tmp/src.txt");
        std::fs::write(&src, b"Hello, World!").unwrap();

        let my_sub_config_dir = base_dirs
            .config_home()
            .join("my_sub_dir")
            .copy("hello.txt", &src)
            .unwrap();

        assert!(my_sub_config_dir.starts_with("/tmp/config/my_sub_dir/hello.txt"));
        std::fs::remove_dir_all(my_sub_config_dir.parent().unwrap()).unwrap();
    }

    #[test]
    #[serial]
    fn copy_file_exists() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::with_prefix("my_sub_dir").unwrap();
        let my_sub_config_dir = base_dirs.config_home().create().unwrap();
        let existing_file = my_sub_config_dir.join("old.txt");
        std::fs::write(&existing_file, b"Old Content").unwrap();
        let new_file = my_sub_config_dir.join("new.txt");
        std::fs::write(&new_file, b"New Content").unwrap();

        let my_sub_config_dir = base_dirs.config_home().copy("old.txt", &new_file).unwrap();

        let content = std::fs::read(&existing_file).unwrap();
        assert_eq!(content, b"Old Content");
        std::fs::remove_dir_all(my_sub_config_dir.parent().unwrap()).unwrap();
    }

    #[test]
    #[serial]
    fn write_over_file_exists() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let my_file = base_dirs
            .config_home()
            .write_over("hello.txt", b"First")
            .unwrap();
        assert!(my_file.starts_with("/tmp/config/hello.txt"));

        // Write again, should overwrite
        let my_file2 = base_dirs
            .config_home()
            .write_over("hello.txt", b"Second")
            .unwrap();
        assert_eq!(my_file, my_file2);

        let content = std::fs::read(&my_file).unwrap();
        assert_eq!(content, b"Second");

        std::fs::remove_file(my_file).unwrap();
    }

    #[test]
    #[serial]
    fn write_over_file_not_exists() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let my_file = base_dirs
            .config_home()
            .write_over("hello.txt", b"Hello, World!")
            .unwrap();

        assert!(my_file.starts_with("/tmp/config/hello.txt"));
        std::fs::remove_file(my_file).unwrap();
    }

    #[test]
    #[serial]
    fn write_file_exists() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let my_file = base_dirs
            .config_home()
            .write_over("hello.txt", b"First")
            .unwrap();

        // Write using `.write`, should NOT overwrite
        let my_file2 = base_dirs
            .config_home()
            .write("hello.txt", b"Second")
            .unwrap();
        assert_eq!(my_file, my_file2);

        let content = std::fs::read(&my_file).unwrap();
        assert_eq!(content, b"First"); // Should still be "First"

        std::fs::remove_file(my_file).unwrap();
    }

    #[test]
    #[serial]
    fn write_file_nots_exists() {
        set_var("XDG_CONFIG_HOME", "/tmp/config");
        let base_dirs = BaseDirs::new().unwrap();

        let path = base_dirs
            .config_home()
            .write("hello.txt", b"Hello, World!")
            .unwrap();

        assert!(path.starts_with("/tmp/config/hello.txt"));
        let content = std::fs::read(&path).unwrap();
        assert_eq!(content, b"Hello, World!");

        std::fs::remove_file(path).unwrap();
    }
}
