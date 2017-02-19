use std;
use std::path::Path;
use std::fmt::{
  Display,
  Formatter,
};

use ::KVStore;
use ::cmd::Command;

pub enum UiError {
  KvStoreNotExisting(String),
  NoValueForKey(String),
  KvError(::KVError),
  CmdError(::cmd::Error),
  UnknownError(String),
}

impl Display for UiError {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match *self {
      UiError::KvStoreNotExisting(ref path) => write!(f, "kv store at {} not existing. Consider creating with the 'init' command", path),
      UiError::NoValueForKey(ref key) => write!(f, "no value for key {}", key),
      UiError::KvError(ref e) => e.fmt(f),
      UiError::CmdError(ref e) => e.fmt(f),
      UiError::UnknownError(ref msg) => write!(f, "unknown error: {}", msg),
    }
  }
}

impl From<::cmd::Error> for UiError {
  fn from(e: ::cmd::Error) -> Self {
    UiError::CmdError(e)
  }
}

impl From<::KVError> for UiError {
  fn from(e: ::KVError) -> Self {
    UiError::KvError(e)
  }
}

pub enum UiResult {
  StringValueResult(String),
  StringListResult(Vec<String>),
  Ok,
}


impl Display for UiResult {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match *self {
      UiResult::StringValueResult(ref value) => write!(f, "{}", value),
      UiResult::StringListResult(ref strings) => {
        if strings.len() == 0 {
          write!(f, "(empty list)")
        } else {
          write!(f, "{}", strings.join("\n"))
        }
      },
      UiResult::Ok => write!(f, "ok"),
    }
  }
}

impl UiResult {
  pub fn ok(_: ()) -> Result<UiResult> {
    Ok(UiResult::Ok)
  }
}

type Result<T> = std::result::Result<T, UiError>;

#[derive(Debug)]
pub struct Ui {
  program: String,
  store_file: String,
  enumerate_list: bool,
}

impl Ui {
  pub fn new(program: String, store_file: String, enumerate_list: bool) -> Ui {
    Ui {
      program: program,
      store_file: store_file,
      enumerate_list: enumerate_list,
    }
  }

  fn load_or_create_kvstore(&self, store_path: &Path, is_init: bool) -> Result<KVStore> {
    if !store_path.exists() {
      if !is_init {
        return Err(UiError::KvStoreNotExisting(store_path.to_str().unwrap_or("<invalid path>").to_string()));
      }

      return Ok(KVStore::new());
    }

    return Ok(::KVStore::read_from_file(store_path)?);
  }

  pub fn run(&self, args: Vec<String>) -> Result<UiResult> {
    let store_path = Path::new(&self.store_file);

    let command = Command::from_strings(args)?;

    let mut kvs = self.load_or_create_kvstore(store_path, command == Command::Init)?;

    let result = self.interpret(&mut kvs, &command)?;

    kvs.write_to_file(store_path)?;

    Ok(result)
  }

  fn interpret(&self, kvs: &mut KVStore, command: &Command) -> Result<UiResult> {
    match *command {
      Command::Init => Ok(UiResult::Ok),
      Command::PutString(ref key, ref value) => UiResult::ok(kvs.put(key, value)),
      Command::Get(ref key) => self.get(key, kvs),
      Command::ListKeys => self.list_keys(kvs),
      Command::CreateEmptyList(ref key) => UiResult::ok(kvs.create_empty_list(key)),
      Command::PushListValue(ref key, ref value) => UiResult::ok(kvs.push_value(key, value)?),
      Command::PopListValue(ref key) => Ok(UiResult::StringValueResult(kvs.pop_value(key)?)),
      Command::Drop(ref key) => self.drop(key, kvs),

      ref cmd => Err(UiError::UnknownError(format!("not implemented yet: {:?}", cmd)))
    }
  }

  fn drop(&self, key: &String, kvs: &mut KVStore) -> Result<UiResult> {
    match kvs.drop(key) {
      Some(v) => Ok(self.to_result(&v)),
      None => Ok(UiResult::Ok),
    }
  }

  fn list_keys(&self, kvs: &KVStore) -> Result<UiResult> {
    Ok(UiResult::StringListResult(kvs.get_keys().iter().map(|x| x.to_string()).collect()))
  }

  fn get(&self, key: &String, kvs: &KVStore) -> Result<UiResult> {
    let value = kvs.get(key).ok_or(UiError::NoValueForKey(key.clone()))?;

    Ok(self.to_result(value))
  }

  fn to_result(&self, value: &::Value) -> UiResult {
    match *value {
      ::Value::StringValue(ref val) => UiResult::StringValueResult(val.clone()),
      ::Value::ListValue(ref list) => UiResult::StringListResult(self.prepare_list_result(list)),
    }
  }

  fn prepare_list_result(&self, list: &Vec<String>) -> Vec<String> {
    if self.enumerate_list {
      let mut i = 0;
      list.iter().map(|x| {
        i += 1;
        format!("{}: {}", i, x)
      }).collect()
    } else {
      list.clone()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_construct() {
    let ui = Ui::new("program".to_string(), "test".to_string(), false);
  }
}
