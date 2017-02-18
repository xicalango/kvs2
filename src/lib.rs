#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod cmd;

use std::collections::HashMap;
use std::str::FromStr;

type Error = String;

type Result<V> = std::result::Result<V, Error>;

#[derive(Serialize, Deserialize, Debug)]
pub struct KVStore {
  content: HashMap<String, Value>
}

impl FromStr for KVStore {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    serde_json::from_str(s).map_err(|e| e.to_string())
  }
}

impl KVStore {
  pub fn new() -> KVStore {
    KVStore {
      content: HashMap::new()
    }
  }

  pub fn serialize(&self) -> Result<String> {
    serde_json::to_string(self).map_err(|e| e.to_string())
  }

  fn put_value<S: ToString>(&mut self, key: S, value: Value) {
    self.content.insert(key.to_string(), value);
  }

  fn get_mut<S: ToString>(&mut self, key: S) -> Result<&mut Value> {
    self.content.get_mut(&key.to_string()).ok_or("no key".to_string())
  }

  fn get_mut_list<S: ToString>(&mut self, key: S) -> Result<&mut Vec<String>> {
    let key = key.to_string();
    let kv_value = self.get_mut(&key)?;

    if let &mut Value::ListValue(ref mut list) = kv_value {
      Ok(list)
    } else {
      Err(format!("value at {} not a list", &key))
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
    let kv_value = self.get_mut(key)?;

    if let &mut Value::ListValue(ref mut list) = kv_value {
      Ok(list.push(value.to_string()))
    } else {
      Err("not a list".to_string())
    }
  }

  pub fn push_all_values<KS: ToString, VS: ToString>(&mut self, key: KS, values: Vec<VS>) -> Result<()> {
    let kv_value = self.get_mut(key)?;

    let mut string_values = values.iter().map(|x| x.to_string()).collect();

    if let &mut Value::ListValue(ref mut list) = kv_value {
      Ok(list.append(&mut string_values))
    } else {
      Err("not a list".to_string())
    }
  }

  pub fn pop_value<KS: ToString>(&mut self, key: KS) -> Result<String> {
    let list_value = self.get_mut_list(key)?;

    list_value.pop().ok_or("list is empty".to_string())
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
