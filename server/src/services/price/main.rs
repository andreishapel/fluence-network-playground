// use marine_rs_sdk::marine;
// use marine_rs_sdk::module_manifest;
//
// module_manifest!();
//
// pub fn main() {}
//
// #[marine]
// pub struct Response {
//   crypto_currency: String,
//   fiat_currency: String,
//   price: String,
// }
//
// #[marine]
// pub fn hash_text(algorithm: &str, text: &str) -> Response {
//   Response {
//     crypto_currency: format!("{}", 1),
//     fiat_currency: format!("{}", 2),
//     price: format!("{}", 3),
//   }
// }

#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;

module_manifest!();

pub fn main() {}

#[marine]
pub fn download(url: String) -> String {
  let result = unsafe {
    curl(vec![url])
  };

  String::from_utf8(result.stdout).unwrap()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
  fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}
