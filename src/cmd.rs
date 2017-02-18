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
    let split = s.split(' ').map(|x| x.to_string()).collect();

    Command::from_strings(split)
  }
}

impl Command {
  pub fn from_strings(strings: Vec<String>) -> Result<Self> {

    match strings[0].as_str() {
      "init" => Ok(Command::Init),

      "put" => assert_length(&strings, 3).map(|v| Command::PutString(v[1].clone(), v[2].clone())),
      "drop" => assert_length(&strings, 2).map(|v| Command::DropString(v[1].clone())),

      "createEmptyList" => assert_length(&strings, 2).map(|v| Command::CreateEmptyList(v[1].clone())),
      "pushListValue" => assert_length(&strings, 3).map(|v| Command::PushListValue(v[1].clone(), v[2].clone())),
      "popListValue" => assert_length(&strings, 2).map(|v| Command::PopListValue(v[1].clone())),
      "dropList" => assert_length(&strings, 2).map(|v| Command::DropList(v[1].clone())),
      "clearList" => assert_length(&strings, 2).map(|v| Command::ClearList(v[1].clone())),

      "get" => assert_length(&strings, 2).map(|v| Command::Get(v[1].clone())),

      cmd => Err(Error::InvalidCommand(cmd.to_string()))
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
      assert_eq!(cmd, "invalid".to_string());
    } else {
      assert!(false);
    }
  }

  #[test]
  fn test_from_vec() {
    let strings = "put bla gna".split(' ').map(|x| x.to_string()).collect();
    let cmd = Command::from_strings(strings).unwrap();

    assert_eq!(cmd, Command::PutString("bla".to_string(), "gna".to_string()));
  }

}
