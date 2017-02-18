use std::str::FromStr;

use std;

#[derive(Debug)]
pub enum Error {
  InvalidCommand(String),
  TooFewArguments(usize, usize),
  UnknownError(String),
}

type Result<V> = std::result::Result<V, Error>;

#[derive(Debug, PartialEq, Eq)]
enum Command {
  Init,

  PutString(String, String),
  DropString(String),

  CreateEmptyList(String),
  PushListValue(String, String),
  PopListValue(String),
  DropList(String),
  ClearList(String),

  Get(String),
}

fn assert_length<T>(v: &Vec<T>, l: usize) -> Result<&Vec<T>> {
  if v.len() >= l {
    Ok(v)
  }  else {
    Err(Error::TooFewArguments(l, v.len()))
  }
}

impl FromStr for Command {
  type Err = Error;

  fn from_str(s: &str) -> Result<Self> {
    let split: Vec<&str> = s.split(' ').collect();

    match split[0] {
      "init" => Ok(Command::Init),
      "put" => assert_length(&split, 3).map(|v| Command::PutString(v[1].to_string(), v[2].to_string())),
      _ => Err(Error::InvalidCommand(s.to_string()))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_read_init() {
    let cmd = Command::from_str("init").unwrap();

    assert_eq!(cmd, Command::Init);
  }

  #[test]
  fn test_put_string() {
    let cmd = Command::from_str("put bla gna").unwrap();

    assert_eq!(cmd, Command::PutString("bla".to_string(), "gna".to_string()));
  }

  #[test]
  fn test_put_string_wrong_fmt() {
    let err = Command::from_str("put bla,gna").err().unwrap();

    if let Error::TooFewArguments(expected, actual) = err {
      assert_eq!(expected, 3);
      assert_eq!(actual, 2);
    } else {
      assert!(false);
    }
  }

  #[test]
  fn test_invalid_command() {
    let err = Command::from_str("invalid test").err().unwrap();

    if let Error::InvalidCommand(cmd) = err {
      assert_eq!(cmd, "invalid test".to_string());
    } else {
      assert!(false);
    }
  }

}
