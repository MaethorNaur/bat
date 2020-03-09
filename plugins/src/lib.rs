extern crate libloading;
extern crate rmp_serde as rmps;
extern crate serde;
#[macro_use]
extern crate log;
mod arg;
pub mod configuration;
mod macros;
mod plugins;
pub use crate::plugins::PluginManager;
pub mod gherkin;
pub use arg::*;
pub use plugins::{List, PluginResult};
