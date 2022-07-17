use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_sqlite_connector;
use rand::Rng;
use serde::{Serialize};
use serde_json;
use sql_lexer;

#[macro_use]
extern crate fstrings;

module_manifest!();

pub fn main() {}

#[derive(Serialize)]
pub struct DatabaseEntity {
  id: u64,
  first_name: String,
  last_name: String,
}

#[marine]
pub struct Response {
  database_id: u16,
}

#[marine]
pub struct ReadResponse {
  database_id: u16,
}

fn get_database_file_path(id: u16) -> String {
  f!("/tmp/{id}.sqlite")
}

#[marine]
pub fn create() -> Response {
  let database_id = rand::thread_rng().gen_range(0..9999);
  let database_file_path = get_database_file_path(database_id);

  let connection = marine_sqlite_connector::open(database_file_path).unwrap();
  let query = f!("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY AUTOINCREMENT, first_name TEXT, last_name TEXT);");
  let sanitized_query = sql_lexer::sanitize_string(query);
  let connection_execution = connection.execute(sanitized_query);

  Response {
    database_id,
  }
}

#[marine]
pub fn read(database_id: u16) -> ReadResponse {
  // let mut entities = vec![];

  let database_file_path = get_database_file_path(database_id);
  let connection = marine_sqlite_connector::open(database_file_path).unwrap();
  let query = f!("SELECT * FROM users");
  let sanitized_query = sql_lexer::sanitize_string(query);

  let mut cursor = connection
      .prepare(sanitized_query)
      .unwrap();

  // let mut cursor = connection
  //     .prepare(sanitized_query)
  //     .unwrap()
  //     .cursor();

  //
  // while let Some(row) = cursor.next().unwrap() {
  //   let id = row[0].as_integer().unwrap() as u64;
  //   let first_name = row[1].as_string().unwrap().to_string();
  //   let last_name = row[2].as_string().unwrap().to_string();
  //
  //   let database_entity = DatabaseEntity {
  //     id,
  //     first_name,
  //     last_name,
  //   };
  //
  //   entities.push(database_entity);
  // }
  //
  // let serialized_entities = serde_json::to_string(&entities).unwrap();
  //
  // ReadResponse {
  //   database_id,
  //   entities: serialized_entities,
  // }
}

#[marine]
pub fn insert(database_id: u16, first_name: String, last_name: String) -> Response {
  let database_file_path = get_database_file_path(database_id);
  let connection = marine_sqlite_connector::open(&database_file_path).unwrap();
  let query = f!("INSERT INTO users (first_name, last_name) VALUES ('FIRST_NAME', 'LAST_NAME');")
    .replace("FIRST_NAME", &first_name)
    .replace("LAST_NAME", &last_name);

  let sanitized_query = sql_lexer::sanitize_string(query);
  connection.execute(sanitized_query);

  Response {
    database_id,
  }
}
//
// #[marine]
// pub fn delete(database_id: u16, id: u64) -> Response {
//   let database_file_path = get_database_file_path(database_id);
//   let connection = marine_sqlite_connector::open(database_file_path).unwrap();
//   let query = f!("DELETE FROM users where id = ID").replace("ID", &id.to_string());
//
//   let sanitized_query = sql_lexer::sanitize_string(query);
//   connection.execute(sanitized_query);
//
//   Response {
//     database_id,
//   }
// }
