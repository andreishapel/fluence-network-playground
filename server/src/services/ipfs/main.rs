use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use ipfs;

module_manifest!();

pub fn main() {}

#[marine]
pub struct Response {
  pub error_message: String,
}

#[marine]
pub fn get_file_hash(content: String) -> String {
  "abc"
}

#[marine]
pub fn write_text_file(content: String) -> String {
  "abc"
}
