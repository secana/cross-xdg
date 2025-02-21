use std::path::{Path, PathBuf};

pub struct BaseDirs {
    home: PathBuf,
    config_home: PathBuf,
    cache_home: PathBuf,
    data_home: PathBuf,
    state_home: PathBuf,
    runtime_dir: Option<PathBuf>,
}

impl BaseDirs {
    pub fn new() -> Result<Self, std::io::Error> {
        Self::internal_new(None)
    }

    pub fn with_prefix(prefix: impl Into<String>) -> Result<Self, std::io::Error> {
        Self::internal_new(Some(prefix.into()))
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    pub fn config_home(&self) -> &Path {
        &self.config_home
    }

    pub fn cache_home(&self) -> &Path {
        &self.cache_home
    }

    pub fn data_home(&self) -> &Path {
        &self.data_home
    }

    pub fn state_home(&self) -> &Path {
        &self.state_home
    }

    pub fn runtime_dir(&self) -> Option<&Path> {
        self.runtime_dir.as_deref()
    }

    pub fn create_config_home(&self) -> Result<&Path, std::io::Error> {
        todo!("Implement this function - do we need to take a path as a parameter?");
        std::fs::create_dir_all(self.config_home())?;
        Ok(self.config_home())
    }

    fn internal_new(prefix: Option<String>) -> Result<Self, std::io::Error> {
        // Ignore deprecation warning, as home_dir was fixed in Rust 1.85 and the warning will be
        // removed in 1.86
        #[allow(deprecated)]
        let tmp_home = std::env::home_dir().ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        ))?;

        Ok(Self {
            config_home: Self::get_config_home(&tmp_home, &prefix),
            cache_home: Self::get_cache_home(&tmp_home, &prefix),
            data_home: Self::get_data_home(&tmp_home, &prefix),
            state_home: Self::get_state_home(&tmp_home, &prefix),
            runtime_dir: Self::get_runtime_dir(&prefix),
            home: tmp_home,
        })
    }

    fn get_cache_home(home: &Path, prefix: &Option<String>) -> PathBuf {
        Self::get_dir(home, "XDG_CACHE_HOME", ".cache", prefix)
    }

    fn get_data_home(home: &Path, prefix: &Option<String>) -> PathBuf {
        Self::get_dir(home, "XDG_DATA_HOME", ".local/share", prefix)
    }

    fn get_state_home(home: &Path, prefix: &Option<String>) -> PathBuf {
        Self::get_dir(home, "XDG_STATE_HOME", ".local/state", prefix)
    }

    fn get_runtime_dir(prefix: &Option<String>) -> Option<PathBuf> {
        let dir = std::env::var("XDG_RUNTIME_DIR").map(PathBuf::from).ok();
        match (&dir, prefix) {
            (Some(dir), Some(prefix)) => Some(dir.join(prefix)),
            _ => dir, // Either both are None or prefix is None
        }
    }

    fn get_config_home(home: &Path, prefix: &Option<String>) -> PathBuf {
        Self::get_dir(home, "XDG_CONFIG_HOME", ".config", prefix)
    }

    fn get_dir(home: &Path, xdg_var: &str, default: &str, prefix: &Option<String>) -> PathBuf {
        let xdg_dir = std::env::var(xdg_var).ok();

        let dir = if let Some(dir) = xdg_dir {
            PathBuf::from(dir)
        } else {
            home.join(default)
        };

        match prefix {
            Some(prefix) => dir.join(prefix),
            None => dir,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    mod without_prefix {
        use super::*;

        #[test]
        #[serial]
        fn config_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_CONFIG_HOME");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.config_home(), &get_home().join(".config"));
        }

        #[test]
        #[serial]
        fn config_home_xdg_set_returns_correct_path() {
            set_var("XDG_CONFIG_HOME", "/tmp/config");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.config_home(), Path::new("/tmp/config"));
        }

        #[test]
        #[serial]
        fn cache_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_CACHE_HOME");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.cache_home(), &get_home().join(".cache"));
        }

        #[test]
        #[serial]
        fn cache_home_xdg_set_returns_correct_path() {
            set_var("XDG_CACHE_HOME", "/tmp/cache");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.cache_home(), Path::new("/tmp/cache"));
        }

        #[test]
        #[serial]
        fn data_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_DATA_HOME");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.data_home(), &get_home().join(".local/share"));
        }

        #[test]
        #[serial]
        fn data_home_xdg_set_returns_correct_path() {
            set_var("XDG_DATA_HOME", "/tmp/data");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.data_home(), Path::new("/tmp/data"));
        }

        #[test]
        #[serial]
        fn state_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_STATE_HOME");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.state_home(), &get_home().join(".local/state"));
        }

        #[test]
        #[serial]
        fn state_home_xdg_set_returns_correct_path() {
            set_var("XDG_STATE_HOME", "/tmp/state");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.state_home(), Path::new("/tmp/state"));
        }

        #[test]
        #[serial]
        fn runtime_dir_no_xdg_set_returns_none() {
            unset_var("XDG_RUNTIME_DIR");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.runtime_dir(), None);
        }

        #[test]
        #[serial]
        fn runtime_dir_xdg_set_returns_correct_path() {
            set_var("XDG_RUNTIME_DIR", "/tmp/runtime");
            let base_dir = BaseDirs::new().unwrap();
            assert_eq!(base_dir.runtime_dir(), Some(Path::new("/tmp/runtime")));
        }
    }

    mod with_prefix {
        use super::*;

        #[test]
        #[serial]
        fn config_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_CONFIG_HOME");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.config_home(), &get_home().join(".config/prefix"));
        }

        #[test]
        #[serial]
        fn config_home_xdg_set_returns_correct_path() {
            set_var("XDG_CONFIG_HOME", "/tmp/config");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.config_home(), Path::new("/tmp/config/prefix"));
        }

        #[test]
        #[serial]
        fn cache_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_CACHE_HOME");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.cache_home(), &get_home().join(".cache/prefix"));
        }

        #[test]
        #[serial]
        fn cache_home_xdg_set_returns_correct_path() {
            set_var("XDG_CACHE_HOME", "/tmp/cache");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.cache_home(), Path::new("/tmp/cache/prefix"));
        }

        #[test]
        #[serial]
        fn data_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_DATA_HOME");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(
                base_dir.data_home(),
                &get_home().join(".local/share/prefix")
            );
        }

        #[test]
        #[serial]
        fn data_home_xdg_set_returns_correct_path() {
            set_var("XDG_DATA_HOME", "/tmp/data");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.data_home(), Path::new("/tmp/data/prefix"));
        }

        #[test]
        #[serial]
        fn state_home_no_xdg_set_returns_correct_path() {
            unset_var("XDG_STATE_HOME");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(
                base_dir.state_home(),
                &get_home().join(".local/state/prefix")
            );
        }

        #[test]
        #[serial]
        fn state_home_xdg_set_returns_correct_path() {
            set_var("XDG_STATE_HOME", "/tmp/state");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.state_home(), Path::new("/tmp/state/prefix"));
        }

        #[test]
        #[serial]
        fn runtime_dir_no_xdg_set_returns_none() {
            unset_var("XDG_RUNTIME_DIR");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(base_dir.runtime_dir(), None);
        }

        #[test]
        #[serial]
        fn runtime_dir_xdg_set_returns_correct_path() {
            set_var("XDG_RUNTIME_DIR", "/tmp/runtime");
            let base_dir = BaseDirs::with_prefix("prefix").unwrap();
            assert_eq!(
                base_dir.runtime_dir(),
                Some(Path::new("/tmp/runtime/prefix"))
            );
        }
    }
    //////////////////////////////
    // Helper functions
    //////////////////////////////

    fn get_home() -> PathBuf {
        #[allow(deprecated)]
        std::env::home_dir().unwrap()
    }

    fn set_var(var: &str, value: &str) {
        unsafe {
            std::env::set_var(var, value);
        }
    }

    fn unset_var(var: &str) {
        unsafe {
            std::env::remove_var(var);
        }
    }
}
