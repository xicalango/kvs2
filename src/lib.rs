#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod cmd;

pub mod ui;

use std::collections::HashMap;
use std::str::FromStr;
use std::io::{
  Read,
  Write,
  BufReader,
  BufWriter,
};

use std::path::Path;
use std::fs::{
  File,
  OpenOptions,
};

use std::fmt::{
  Display,
  Formatter
};

#[derive(Debug)]
pub enum KVError {
  IoError(std::io::Error),
  EncodingError(serde_json::Error),
  UnknownError(String),
}

impl Display for KVError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match *self {
      KVError::IoError(ref e) => write!(f, "{}", e.to_string()),
      KVError::EncodingError(ref e) => write!(f, "{}", e.to_string()),
      KVError::UnknownError(ref msg) => write!(f, "{}", msg),
    }
  }
}

type Result<V> = std::result::Result<V, KVError>;

impl From<serde_json::Error> for KVError {
  fn from(e: serde_json::Error) -> Self {
    KVError::EncodingError(e)
  }
}

impl From<std::io::Error> for KVError {
  fn from(e: std::io::Error) -> Self {
    KVError::IoError(e)
  }
}

pub fn init<P: AsRef<Path>>(path: P) -> Result<KVStore> {
  let file = File::create(path)?;
  let mut writer = BufWriter::new(file);

  let kvs = KVStore::new();
  kvs.write(&mut writer)?;

  Ok(kvs)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KVStore {
  content: HashMap<String, Value>
}

impl FromStr for KVStore {
  type Err = KVError;

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

  pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<KVStore> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    KVStore::read(reader)
  }

  pub fn serialize(&self) -> Result<String> {
    Ok(serde_json::to_string(self)?)
  }

  pub fn write<W: Write>(&self, w: &mut W) -> Result<()> {
    Ok(serde_json::to_writer(w, self)?)
  }

  pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
    let file = OpenOptions::new().write(true).truncate(true).open(path)?;
    let mut writer = BufWriter::new(file);

    self.write(&mut writer)
  }

  fn put_value<S: ToString>(&mut self, key: S, value: Value) -> Option<Value> {
    self.content.insert(key.to_string(), value)
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

  pub fn put_empty_list<S: ToString>(&mut self, key: S) -> Option<Value> {
    let empty_list = Vec::new();
    let value = Value::ListValue(empty_list);

    self.put_value(key.to_string(), value)
  }

  pub fn push_value<KS: ToString, VS: ToString>(&mut self, key: KS, value: VS) -> Result<()> {
    let list = self.get_mut_list(key)?;
    Ok(list.push(value.to_string()))
  }

  pub fn push_all_values<KS: ToString, VS: ToString>(&mut self, key: KS, values: Vec<VS>) -> Result<()> {
    let list = self.get_mut_list(key)?;
    let mut string_values = values.iter().map(ToString::to_string).collect();
    Ok(list.append(&mut string_values))
  }

  pub fn pop_value<KS: ToString>(&mut self, key: KS) -> Result<String> {
    let list_value = self.get_mut_list(key)?;

    list_value.pop().ok_or(KVError::UnknownError("list is empty".to_string()))
  }

  pub fn get_keys(&self) -> Vec<&String> {
    self.content.keys().collect()
  }

  pub fn drop<KS: ToString>(&mut self, key: KS) -> Option<Value> {
    self.content.remove(&key.to_string())
  }

  pub fn has_key<KS: ToString>(&self, key: KS) -> bool {
    self.content.contains_key(&key.to_string())
  }

  pub fn get_value_type<KS: ToString>(&self, key: &KS) -> Option<ValueType> {
    self.get(key).map(Value::get_type)
  }
}

pub enum ValueType {
  String,
  List,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Value {
  StringValue(String),
  ListValue(Vec<String>)
}

impl Value {
  pub fn get_type(&self) -> ValueType {
    match *self {
      Value::StringValue(_) => ValueType::String,
      Value::ListValue(_) => ValueType::List,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_create() {
    KVStore::new();
  }
}
