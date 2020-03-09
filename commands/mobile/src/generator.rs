use crate::{v2, v3_0};
use openapi::Error;
use openapi::OpenApi;
use plugins::gherkin;
pub fn generate(input: &str) -> Result<gherkin::Feature, Error> {
    openapi::from_path(input).map(|spec| match spec {
        OpenApi::V3_0(spec) => v3_0::new(spec),
        OpenApi::V2(spec) => v2::new(spec),
    })
}
