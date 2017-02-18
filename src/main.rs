extern crate kvs2;

fn main() {

  let mut kvs = kvs2::KVStore::new();

  kvs.put("test", "test");

  kvs.create_empty_list("test2");
  kvs.push_value("test2", "list").unwrap();
  kvs.push_value("test2", "list_2").unwrap();

  let serialized = kvs.serialize().unwrap();

  println!("{}", serialized);

}
