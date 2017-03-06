extern crate kvs2;
extern crate getopts;

use kvs2::ui::{
  Ui,
  UiResult,
};

use kvs2::hooks;

use std::env;

use std::fmt::Display;

use std::process::exit;

use std::path;

use getopts::Options;

fn die<D: Display>(error: &D) {
    println!("Error: {}", error);
    exit(1);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut opts = Options::new();

  opts.optopt("s", "store", "kv store to use", "STORE");
  opts.optflag("n", "number", "enumerate list values");

  let args = match opts.parse(&args[1..]) {
    Ok(m) => m,
    Err(f) => panic!(f.to_string())
  };

  let store_file = args.opt_str("s").unwrap_or(".kvs.json".to_string());

  let store_file_clone = store_file.clone();
  let hooks_dir = path::Path::new(&store_file_clone).parent().unwrap_or(path::Path::new("./"));

  let hooks = hooks::Hooks::load_from_dir(hooks_dir);

  let ui = Ui::new(program, store_file, args.opt_present("n"), hooks);

  match ui.run(args.free) {
    Ok(UiResult::Ok) => (),
    Ok(result) => println!("{}", result),
    Err(err) => die(&err),
  };
}

