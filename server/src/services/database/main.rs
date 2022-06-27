use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector::{Value};
use sql_lexer;

#[macro_use]
extern crate fstrings;

module_manifest!();

pub fn main() {}

#[marine]
pub struct Response {
  error_message: String,
}

#[marine]
pub fn init(table_name: String) -> String {
  let connection = marine_sqlite_connector::open(":memory:").unwrap();
  let query = f!("CREATE TABLE usersKSDKLASKJDJKLASKLJDKLJSAD (first_name TEXT, last_name TEXT);");

  let sanitized_query = sql_lexer::sanitize_string(query);
  let connection_status = connection
    .execute(sanitized_query);

  match connection_status  {
    Ok(_) => {
      "success".to_string()
    },
    Err(error) => {
      error.to_string()
    },
  }
}

#[marine]
pub fn insert(table_name: String, first_name: String, last_name: String) -> String {
  let connection = marine_sqlite_connector::open(":memory:").unwrap();
  let query = f!("INSERT INTO usersKSDKLASKJDJKLASKLJDKLJSAD VALUES ('FIRST_NAME', LAST_NAME);")
    .replace("FIRST_NAME", &first_name)
    .replace("LAST_NAME", &last_name);

  let sanitized_query = sql_lexer::sanitize_string(query);
  let connection_status = connection
      .execute(sanitized_query);

  match connection_status  {
    Ok(_) => {
      "".to_string()
    },
    Err(error) => {
      error.to_string()
    },
  }
}

#[marine]
pub fn read(table_name: String, first_name: String) -> String {
  let a = String::from("");

  let connection = marine_sqlite_connector::open(":memory:").unwrap();
  let query = f!("SELECT * FROM TABLE_NAME WHERE first_name = ?")
    .replace("TABLE_NAME", &table_name)
    .replace("FIRST_NAME", &first_name);

  let sanitized_query = sql_lexer::sanitize_string(query);
  let mut cursor = connection
      .prepare(sanitized_query)
      .unwrap()
      .cursor();

  cursor.bind(&[Value::String(first_name)]).unwrap();
  while let Some(row) = cursor.next().unwrap() {
    let row1 = row[0].as_string().unwrap();
    let row2 = row[1].as_string().unwrap();

    row1.to_string().push_str(row2);
    a.to_string().push_str(row1);
  }

  a.to_string()
}
//
// #[marine]
// pub fn update(first_name: String) -> bool {
//   false
// }
//
// #[marine]
// pub fn delete(first_name: String) -> bool {
//   false
// }