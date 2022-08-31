use marine_rs_sdk::{marine, module_manifest};
use rand::{Rng};
use std::io::prelude::*;
use std::io::Error;
use std::fs::File;

module_manifest!();

fn main() {}

#[marine]
pub struct IPFSResponse {
  output: String,
  error: String,
}

#[marine]
pub struct UploadResponse {
  pub output: String,
  pub error: String,
}

fn generate_random_number() -> i32 {
  let mut thread = rand::thread_rng();
  thread.gen_range(0..99)
}

fn create_temp_file(image_base64: String) -> Result<String, Error> {
  let random_number = generate_random_number();
  let temp_file_path = format!("{:#?}.ipfs.txt", random_number);
  println!("@@@@@, {}", temp_file_path);

  let mut buffer = File::create(&temp_file_path)?;
  let image_bytes = image_base64.as_bytes();
  buffer.write_all(image_bytes)?;

  Ok(temp_file_path)
}

#[marine]
pub fn upload_image(ipfs_address: String, image_base64: String) -> UploadResponse {
  let file_path = create_temp_file(image_base64).unwrap();
  println!("####, {}", file_path);
  println!("$$$$, {}", ipfs_address);

  let ipfs_cmd = vec!["--api".to_string(), ipfs_address, "cat".to_string()];
  let ipfs_response = unsafe { ipfs_request(ipfs_cmd) };

  UploadResponse {
    output: ipfs_response.output,
    error: ipfs_response.error,
  }
}

#[marine]
#[link(wasm_import_module = "ipfs_adapter")]
extern "C" {
  pub fn ipfs_request(cmd: Vec<String>) -> IPFSResponse;
}
