#![allow(dead_code)]

use std::path::PathBuf;

pub(crate) fn get_home() -> PathBuf {
    #[allow(deprecated)]
    std::env::home_dir().unwrap()
}

pub(crate) fn set_var(var: &str, value: &str) {
    unsafe {
        std::env::set_var(var, value);
    }
}

pub(crate) fn unset_var(var: &str) {
    unsafe {
        std::env::remove_var(var);
    }
}
