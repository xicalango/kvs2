extern crate kvs2;
extern crate getopts;

use kvs2::{
  KVStore,
  Value,
};
use kvs2::cmd::Command;

use std::path::Path;

use std::env;

use getopts::Options;

fn get(key: &String, kvs: &KVStore) {
  match kvs.get(key) {
    Some(&Value::StringValue(ref val)) => println!("{}", val),
    None => println!("invalid key: {}", key),
    _ => panic!("")
  }
}

fn interpret(command: &Command, kvs: &mut KVStore) {
  match *command {
    Command::PutString(ref key, ref value) => kvs.put(key, value),
    Command::Get(ref key) => get(key, kvs),
    _ => unimplemented!()
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();

  opts.optopt("s", "store", "kv store to use", "STORE");

  let args = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string())
  };

  let store_file = args.opt_str("s").unwrap_or(".kvs.json".to_string());

  let command = match Command::from_strings(args.free) {
    Ok(cmd) => cmd,
    Err(e) => panic!(e.to_string())
  };

  let store_path = Path::new(&store_file);

  if !store_path.exists() {
    if command != Command::Init {
      panic!(format!("store file {} does not exist. To create call with '{} init'", store_file, &program));
    }

    kvs2::init(&store_path).unwrap();
    println!("successfully initialized at {}", store_file);
    return;
  }

  let mut kvs = KVStore::read_from_file(store_path).unwrap();

  interpret(&command, &mut kvs);

  kvs.write_to_file(store_path).unwrap();

}
