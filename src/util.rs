use std::env;
use std::env::VarError;
use std::ffi::{OsStr, OsString};

pub fn get_env_var_array(array: &[&'static str; 2]) -> Result<String, OsString>{
    get_env_var(array[0], ||String::from(array[1]))
}
pub fn get_env_var<K: AsRef<OsStr>>(key: K, default: impl FnOnce() -> String) -> Result<String, OsString>{
    match env::var(key){
        Ok(val) => Ok(val),
        Err(error) => match error {
            VarError::NotPresent => Ok(default()),
            VarError::NotUnicode(string) => Err(string),
        }
    }
}
