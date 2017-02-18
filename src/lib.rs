#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod cmd;

use std::collections::HashMap;
use std::str::FromStr;
use std::io::{
  Read,
  Write,
};

use std::fmt::{
  Display,
  Formatter
};

#[derive(Debug)]
pub enum KVError {
  UnknownError(String)
}

impl Display for KVError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match *self {
      KVError::UnknownError(ref msg) => write!(f, "{}", msg)
    }
  }
}

type Error = KVError;

type Result<V> = std::result::Result<V, Error>;

impl From<serde_json::Error> for Error {
  fn from(e: serde_json::Error) -> Self {
    KVError::UnknownError(e.to_string())
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KVStore {
  content: HashMap<String, Value>
}

impl FromStr for KVStore {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    Ok(serde_json::from_str(s)?)
  }
}

impl ToString for KVStore {
  fn to_string(&self) -> String {
    self.serialize().unwrap()
  }
}

impl KVStore {
  pub fn new() -> KVStore {
    KVStore {
      content: HashMap::new()
    }
  }

  pub fn read<R: Read>(r: R) -> Result<KVStore> {
    Ok(serde_json::from_reader(r)?)
  }

  pub fn serialize(&self) -> Result<String> {
    Ok(serde_json::to_string(self)?)
  }

  pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
    Ok(serde_json::to_writer(w, self)?)
  }

  fn put_value<S: ToString>(&mut self, key: S, value: Value) {
    self.content.insert(key.to_string(), value);
  }

  fn get_mut<S: ToString>(&mut self, key: S) -> Result<&mut Value> {
    self.content.get_mut(&key.to_string()).ok_or(KVError::UnknownError("no key".to_string()))
  }

  fn get_mut_list<S: ToString>(&mut self, key: S) -> Result<&mut Vec<String>> {
    let key = key.to_string();
    let kv_value = self.get_mut(&key)?;

    if let &mut Value::ListValue(ref mut list) = kv_value {
      Ok(list)
    } else {
      Err(KVError::UnknownError(format!("value at {} not a list", &key)))
    }
  }

  pub fn get<S: ToString>(&self, key: &S) -> Option<&Value> {
    self.content.get(&key.to_string())
  }

  pub fn put<KS: ToString, VS: ToString>(&mut self, key: KS, value: VS) {
    let kv_value = Value::StringValue(value.to_string());
    self.put_value(key, kv_value);
  }

  pub fn create_empty_list<S: ToString>(&mut self, key: S) {
    let empty_list = Vec::new();
    let value = Value::ListValue(empty_list);

    self.put_value(key.to_string(), value);
  }

  pub fn push_value<KS: ToString, VS: ToString>(&mut self, key: KS, value: VS) -> Result<()> {
    let list = self.get_mut_list(key)?;
    Ok(list.push(value.to_string()))
  }

  pub fn push_all_values<KS: ToString, VS: ToString>(&mut self, key: KS, values: Vec<VS>) -> Result<()> {
    let list = self.get_mut_list(key)?;
    let mut string_values = values.iter().map(|x| x.to_string()).collect();
    Ok(list.append(&mut string_values))
  }

  pub fn pop_value<KS: ToString>(&mut self, key: KS) -> Result<String> {
    let list_value = self.get_mut_list(key)?;

    list_value.pop().ok_or(KVError::UnknownError("list is empty".to_string()))
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
  StringValue(String),
  ListValue(Vec<String>)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create() {
    let kvs = KVStore::new();
  }
}
