extern crate kvs2;

use kvs2::cmd::Command;

use std::str::FromStr;

use std::env;

fn main() {

  let args = env::args();

  let command = Command::from_strings(args.skip(1).collect()).unwrap();

  if let Command::Init = command {

  }

  let mut kvs = kvs2::KVStore::new();

  kvs.put("test", "test");

  kvs.create_empty_list("test2");
  kvs.push_value("test2", "list").unwrap();
  kvs.push_value("test2", "list_2").unwrap();
  kvs.push_all_values("test2", vec!["testa", "testb"]).unwrap();

  let val = kvs.pop_value("test2").unwrap();

  println!("popped: {}", val);

  let serialized = kvs.serialize().unwrap();

  println!("{}", serialized);

  let deser = kvs2::KVStore::from_str(&serialized).unwrap();

  println!("{:?}", deser.get(&"test2").unwrap());
}
