use openapi::v2;
use plugins::gherkin::*;
pub fn new(openapi: v2::Spec) -> Feature {
    Feature::new(openapi.info.title.unwrap_or_else(|| "No title".to_string()))
}
