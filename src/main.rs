extern crate kvs2;

use std::str::FromStr;

fn main() {

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
