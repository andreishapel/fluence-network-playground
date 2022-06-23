#![allow(improper_ctypes)]

use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;

module_manifest!();

pub fn main() {}

#[marine]
pub fn download(url: String) -> String {
  let result = unsafe { curl(vec![url]) };
  let output = result.stdout;
  String::from_utf8(output).unwrap()
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
  fn curl(cmd: Vec<String>) -> MountedBinaryResult;
}