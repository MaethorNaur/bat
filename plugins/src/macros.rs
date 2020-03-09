#[macro_export]
macro_rules! on_plugin_load {
    ($body:expr) => {
        #[no_mangle]
        pub extern "C" fn on_plugin_load() {
            $body
        }
    };
}

#[macro_export]
macro_rules! on_plugin_unload {
    ($body:expr) => {
        #[no_mangle]
        pub extern "C" fn on_plugin_unload() {
            $body
        }
    };
}

#[macro_export]
macro_rules! declare_plugin {
    ($args: expr, $body:expr) => {
        #[no_mangle]
        pub extern "C" fn bat_plugin_name() -> *const std::os::raw::c_char {
            let cstr = std::ffi::CString::new(env!("CARGO_PKG_DESCRIPTION")).unwrap();
            cstr.into_raw() as *const std::os::raw::c_char
        }

        #[no_mangle]
        pub extern "C" fn bat_plugin_command() -> *const std::os::raw::c_char {
            let cstr = std::ffi::CString::new(env!("CARGO_PKG_NAME").to_lowercase()).unwrap();
            cstr.into_raw() as *const std::os::raw::c_char
        }

        #[no_mangle]
        pub extern "C" fn bat_plugin_version() -> *const std::os::raw::c_char {
            let cstr = std::ffi::CString::new(env!("CARGO_PKG_VERSION")).unwrap();
            cstr.into_raw() as *const std::os::raw::c_char
        }

        #[no_mangle]
        pub extern "C" fn bat_plugin_args() -> plugins::List {
            let mut buf = plugins::encode_args($args).into_boxed_slice();
            let data = buf.as_mut_ptr();
            let len = buf.len();
            std::mem::forget(buf);
            plugins::List { data, len }
        }

        #[no_mangle]
        pub extern "C" fn bat_plugin_run(parameters: plugins::List) -> plugins::List {
            let params = unsafe { std::slice::from_raw_parts_mut(parameters.data, parameters.len) };
            let input = plugins::decode_parameters(params.to_vec());
            let result = $body(input);
            match result {
                Some(feature) => {
                    let mut buf = feature.serialise().into_boxed_slice();
                    let data = buf.as_mut_ptr();
                    let len = buf.len();
                    std::mem::forget(buf);
                    plugins::List { data, len }
                }
                None => plugins::List {
                    data: std::ptr::null_mut(),
                    len: 0,
                },
            }
        }
    };
}
