use crate::arg::*;
use crate::gherkin::Feature;
use glob::glob;
use libloading::{Library, Symbol};
use rmps::Serializer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, OsStr};
use std::os::raw::c_char;
use std::path::Path;

#[cfg(unix)]
use libloading::os::unix::Symbol as OsSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as OsSymbol;

#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "*.dll";

#[cfg(target_os = "macos")]
const LIB_EXTENSION: &str = "*.dylib";

#[cfg(target_os = "linux")]
const LIB_EXTENSION: &str = "*.so";

type SString = unsafe extern "C" fn() -> *const c_char;
type Void = unsafe extern "C" fn();
type Run = unsafe extern "C" fn(List) -> List;
type ArgFn = unsafe extern "C" fn() -> List;

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[repr(C)]
pub struct List {
    pub data: *mut u8,
    pub len: usize,
}

#[derive(Default, PartialEq, Deserialize, Serialize)]
pub struct PluginResult {
    filename: String,
    feature: Feature,
}

impl PluginResult {
    pub fn new(filename: &str, feature: Feature) -> Self {
        Self {
            filename: filename.to_string(),
            feature,
        }
    }
    pub fn filename(&self) -> &String {
        &self.filename
    }
    pub fn feature(&self) -> &Feature {
        &self.feature
    }
    pub fn serialise(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        self.serialize(&mut Serializer::new(&mut buf)).unwrap();
        buf
    }

    pub fn from_vec(buf: Vec<u8>) -> Self {
        rmp_serde::from_read_ref(&buf).unwrap()
    }
}

pub struct Plugin {
    on_plugin_unload: Void,
    run: Run,
    version: &'static str,
    name: &'static str,
    command: &'static str,
    args: Vec<Arg>,
}

impl Plugin {
    pub fn name(&self) -> &str {
        self.name
    }

    pub fn command(&self) -> &str {
        self.command
    }

    pub fn args(&self) -> String {
        self.args
            .clone()
            .iter()
            .map(|s| (*s.arg()).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }
    pub fn args_name(&self) -> Vec<String> {
        self.args
            .clone()
            .iter()
            .map(|s| (*s.name()).to_string())
            .collect()
    }
    pub fn version(&self) -> &str {
        self.version
    }

    pub fn on_plugin_unload(&self) {
        unsafe { (self.on_plugin_unload)() }
    }

    pub fn run(&self, params: HashMap<String, String>) -> Option<PluginResult> {
        let mut buf = rmp_serde::to_vec(&params).unwrap().into_boxed_slice();
        let data = buf.as_mut_ptr();
        let len = buf.len();
        let ptr = List { data, len };
        unsafe {
            let buf = (self.run)(ptr);
            if buf.len == 0 {
                None
            } else {
                Some(std::slice::from_raw_parts_mut(buf.data, buf.len))
            }
        }
        .map(|slice| {
            let result = PluginResult::from_vec(slice.to_vec());
            let slice_ptr = slice.as_mut_ptr();
            unsafe {
                Box::from_raw(slice_ptr);
            }
            result
        })
    }
}

#[derive(Default)]
pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    loaded_libraries: HashMap<String, Box<Library>>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: HashMap::new(),
            loaded_libraries: HashMap::new(),
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn get_command<P>(&self, mut predicate: P) -> Option<&Plugin>
    where
        P: FnMut(&String) -> bool,
    {
        self.plugins
            .iter()
            .find(|(name, _)| predicate(name))
            .map(|(_, plugin)| plugin)
    }

    pub fn list_commands(&self) -> Vec<(&str, &str, &str, &str)> {
        self.plugins.values().fold(Vec::new(), |mut acc, plugin| {
            let t = (
                plugin.name,
                plugin.version,
                plugin.command,
                string_to_static_str(plugin.args()),
            );
            acc.push(t);
            acc
        })
    }
    pub fn load_plugins<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), String> {
        let path = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(Path::new(&filename))
            .canonicalize()
            .map_err(|e| e.to_string())
            .unwrap();
        trace!("Reading from {:?}", path);
        let pattern = path.join(LIB_EXTENSION);
        for entry in glob(pattern.to_str().unwrap())
            .map_err(|e| e.to_string())
            .unwrap()
        {
            trace!("File: {:?}", entry);
            let path = entry.map_err(|e| e.to_string()).unwrap();
            unsafe {
                self.load_plugin(path).unwrap();
            }
        }
        Ok(())
    }

    #[allow(clippy::missing_safety_doc, unused_variables)]
    pub unsafe fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), String> {
        let lib = Box::new(
            Library::new(filename.as_ref())
                .map_err(|e| e.to_string())
                .unwrap(),
        );

        let on_plugin_unload: OsSymbol<Void> = load_fn(&lib, "on_plugin_unload");
        let on_plugin_load: OsSymbol<Void> = load_fn(&lib, "on_plugin_load");
        let on_plugin_unload: OsSymbol<Void> = load_fn(&lib, "on_plugin_unload");

        let run: OsSymbol<Run> = load_fn(&lib, "bat_plugin_run");

        let version = load_string(&lib, "bat_plugin_version");
        let name = load_string(&lib, "bat_plugin_name");
        let command = load_string(&lib, "bat_plugin_command");

        let args_fn: OsSymbol<ArgFn> = load_fn(&lib, "bat_plugin_args");

        let slice = {
            let buf = (args_fn)();
            std::slice::from_raw_parts_mut(buf.data, buf.len)
        };

        let args: Vec<Arg> = rmp_serde::from_read_ref(&slice.to_vec()).unwrap();

        self.loaded_libraries.insert(String::from(command), lib);

        trace!("Loaded plugin: {} {}", name, version);

        on_plugin_load();

        self.plugins.insert(
            String::from(command),
            Plugin {
                on_plugin_unload: *on_plugin_unload,
                run: *run,
                version,
                name,
                command,
                args,
            },
        );
        Ok(())
    }

    pub fn unload(&mut self) {
        trace!("Unloading plugins");

        for (name, plugin) in self.plugins.drain() {
            trace!("Firing on_plugin_unload for {:?}", name);
            plugin.on_plugin_unload();
        }

        for (_, lib) in self.loaded_libraries.drain() {
            drop(lib);
        }
    }
}

unsafe fn load_fn<T>(lib: &Library, name: &str) -> OsSymbol<T> {
    let func: Symbol<T> = lib
        .get(name.as_bytes())
        .map_err(|_| format!("The `{}` symbol wasn't found.", name))
        .unwrap();

    func.into_raw()
}

unsafe fn load_string(lib: &Library, name: &str) -> &'static str {
    let func = load_fn::<SString>(lib, name);
    CStr::from_ptr(func()).to_str().unwrap()
}
impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}
