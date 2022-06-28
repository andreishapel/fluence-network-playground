use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use serde_json::{Result, Value};

module_manifest!();

pub fn main() {}

#[marine]
pub struct Response {
  pub error_message: String,
}

#[marine]
pub fn create_wallet() -> String {
  "abc"
}
