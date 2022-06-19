use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use base64::{encode};
use md5::{Md5};
use sha2::{Sha256, Digest};

module_manifest!();

pub fn main() {}

fn hash_text_with_algorithm_selected(algorithm: &str, text: &str) -> String {
  match algorithm {
    "md5" => {
      let mut hasher = Md5::new();
      hasher.update(text);
      let result = hasher.finalize();
      encode(result)
    },
    "sha256" => {
      let mut hasher = Sha256::new();
      hasher.update(text);
      let result = hasher.finalize();
      encode(result)
    },
    _ => panic!("No algorithm selected"),
  }
}

#[marine]
pub struct Response {
  pub algorithm: String,
  pub hashed_text: String,
  pub original_text: String,
}

#[marine]
pub fn hash_text(algorithm: &str, text: &str) -> Response {
  let hashed_text = hash_text_with_algorithm_selected(algorithm, text);

  Response {
    algorithm: format!("{}", algorithm),
    hashed_text: format!("{}", hashed_text),
    original_text: format!("{}", text),
  }
}
