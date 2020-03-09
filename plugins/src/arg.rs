use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Arg {
    name: String,
    arg: String,
}

impl Arg {
    pub fn new(name: &str, arg: &str) -> Self {
        Self {
            name: name.to_string(),
            arg: arg.to_string(),
        }
    }
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn arg(&self) -> &String {
        &self.arg
    }
}
pub fn encode_args(args: Vec<Arg>) -> Vec<u8> {
    rmp_serde::to_vec(&args).unwrap()
}
pub fn decode_parameters(buf: Vec<u8>) -> HashMap<String, String> {
    rmp_serde::from_read_ref(&buf).unwrap()
}
