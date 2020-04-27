extern crate openapi;
extern crate slugify;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate plugins;
mod generator;
mod v2;
mod v3_0;

use plugins::{Arg, PluginResult};
use std::collections::HashMap;
use std::path::Path;

fn run(args: HashMap<String, String>) -> Option<PluginResult> {
    let input = args.get("INPUT").unwrap();

    let stem = Path::new(input)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap();
    let filename = format!("{}.feature", stem);
    trace!("Parsing: {}", input);
    match generator::generate(input) {
        Ok(feature) => {
            trace!("Feature generate for {}", input);
            Some(PluginResult::new(&filename, feature))
        }
        Err(err) => {
            error!("error: {}", err);
            None
        }
    }
}

on_plugin_load!({
    pretty_env_logger::init();
    debug!("🔥Mobile loaded");
});

on_plugin_unload!({
    debug!("🔥Mobile unloaded");
});

#[allow(clippy::not_unsafe_ptr_arg_deref)]
declare_plugin!(vec![Arg::new("INPUT", "<INPUT> 'Input file to use'")], run);
