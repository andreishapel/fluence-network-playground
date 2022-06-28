use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
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
pub fn create(table_name: String) -> String {
  let connection = marine_sqlite_connector::open(":memory:").unwrap();
  let query = f!("CREATE TABLE IF NOT EXISTS TABLE_NAME (id INTEGER PRIMARY KEY AUTOINCREMENT, first_name TEXT, last_name TEXT);")
    .replace("TABLE_NAME", &table_name);

  let sanitized_query = sql_lexer::sanitize_string(query);
  let connection_status = connection.execute(sanitized_query);

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
  let query = f!("INSERT INTO TABLE_NAME VALUES ('FIRST_NAME', 'LAST_NAME');")
    .replace("TABLE_NAME", &table_name)
    .replace("FIRST_NAME", &first_name)
    .replace("LAST_NAME", &last_name);

  let sanitized_query = sql_lexer::sanitize_string(query);
  let connection_status = connection.execute(sanitized_query);

  match connection_status  {
    Ok(_) => {
      "".to_string()
    },
    Err(error) => {
      error.to_string()
    },
  }
}

// #[marine]
// pub fn read(table_name: String) -> String {
//   let mut result = json::JsonValue::new_object();
//
//   let connection = marine_sqlite_connector::open(":memory:").unwrap();
//   let query = f!("SELECT * FROM TABLE_NAME").replace("TABLE_NAME", &table_name);
//   let sanitized_query = sql_lexer::sanitize_string(query);
//
//   let a = connection
//    .iterate(sanitized_query, |pairs| {
//      for &(column, value) in pairs.iter() {
//        let column_name: String = column.to_string().into();
//        let value_name: String = value.unwrap().to_string().into();
//        result["column_name"] = "5".into();
//      }
//      true
//    });
//
//   match a  {
//     Ok(_) => {
//       "".to_string()
//     },
//     Err(error) => {
//       return error.to_string()
//     },
//   }
//
//   json::stringify(result.dump())
// }
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
