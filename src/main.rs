extern crate kvs2;
extern crate getopts;

use kvs2::ui::Ui;

use std::env;

use getopts::Options;

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

  let ui = Ui::new(program, store_file, args.opt_present("n"));

  println!("ui: {:?}", ui);

  match ui.run(args.free) {
    Ok(result) => println!("{}", result),
    Err(err) => println!("Error: {}", err),
  };
}
