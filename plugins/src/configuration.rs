use serde::Deserialize;
use std::convert::Into;
use std::ffi::{CStr, CString};

use std::os::raw::c_char;
pub mod ffi {
    use std::os::raw::c_char;
    #[repr(C)]
    pub struct Configuration {
        pub language: *const c_char,
        pub input: *const c_char,
        pub output: *const c_char,
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Configuration {
    #[serde(default = "default_language")]
    pub language: String,
    #[serde(default = "default_input")]
    pub input: String,
    #[serde(default = "default_output")]
    pub output: String,
}

fn from_c_str(c_str_char: *const c_char) -> String {
    let c_str = unsafe {
        assert!(!c_str_char.is_null());
        CStr::from_ptr(c_str_char).to_str().unwrap()
    };
    c_str.to_string()
}

impl ffi::Configuration {
    pub fn from_raw(ptr: *mut ffi::Configuration) -> Configuration {
        let conf = unsafe { &mut *ptr };
        Configuration {
            language: from_c_str(conf.language),
            input: from_c_str(conf.input),
            output: from_c_str(conf.output),
        }
    }
}

impl Into<ffi::Configuration> for Configuration {
    fn into(self) -> ffi::Configuration {
        ffi::Configuration {
            language: CString::new(self.language).unwrap().into_raw(),
            input: CString::new(self.input).unwrap().into_raw(),
            output: CString::new(self.output).unwrap().into_raw(),
        }
    }
}

fn default_language() -> String {
    "scala".to_string()
}

fn default_input() -> String {
    ".".to_string()
}
fn default_output() -> String {
    "out/".to_string()
}
