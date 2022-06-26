use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use serde_json::{Result, Value};

#[macro_use]
extern crate fstrings;

module_manifest!();

pub fn main() {}

#[marine]
pub struct Response {
  pub crypto_currency: String,
  pub fiat_currency: String,
  pub price: f64,
}

#[marine]
pub fn get_price(crypto_currency: String, fiat_currency: String) -> Response {
  let crypto_currency_in_lowercase = crypto_currency.to_lowercase();
  let fiat_currency_in_lowercase = fiat_currency.to_lowercase();

  let url = f!("https://api.coingecko.com/api/v3/simple/price?ids={crypto_currency}&vs_currencies={fiat_currency}");
  let response = unsafe { download(url) };

  let json_response: Result<Value> = serde_json::from_str(&response.clone());
  if json_response.is_err() {
    panic!("Failed to make a HTTP request");
  }

  let json_response_value = json_response.unwrap();
  let value = json_response_value[crypto_currency_in_lowercase.clone()][fiat_currency_in_lowercase.clone()].as_f64();
  if value.is_none() {
    panic!("No prices available");
  }

  let value = value.unwrap();
  Response {
    crypto_currency: format!("{}", &crypto_currency_in_lowercase.clone()),
    fiat_currency: format!("{}", &fiat_currency_in_lowercase.clone()),
    price: value,
  }
}

#[marine]
#[link(wasm_import_module = "curl")]
extern "C" {
  pub fn download(url: String) -> String;
}
