use marine_rs_sdk::{marine, module_manifest, MountedBinaryResult};

module_manifest!();

fn main() {}

#[marine]
pub struct Response {
  output: String,
  error: String,
}

#[marine]
pub fn ipfs_request(cmd: Vec<String>) -> Response{
  let response = unsafe { ipfs(cmd) };

  let stdout = String::from_utf8(response.stdout).unwrap();
  let stderr = String::from_utf8(response.stderr).unwrap();

  if stdout.len() > 0 {
    return Response {
      output: stdout,
      error: "".to_string(),
    };
  }

  return Response {
    output: "".to_string(),
    error: stderr,
  };
}

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
  pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}
