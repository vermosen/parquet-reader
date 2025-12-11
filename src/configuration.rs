use clap::{arg, Command};

#[derive(Debug, Clone)]
pub struct Configuration {
  pub path: String,
  pub debug: bool,  
  pub sep: String,
  pub nohdr: bool,
  pub limit: i32
}

impl Configuration {
  pub fn new() -> Self {
    let args = Command::new("parquet-reader")
      .version("0.0.1")
      .author("jean-mathieu vermosen <vermosen@yahoo.com>")
      .about("A simple parquet file reader")
      .arg(arg!(--path <PATH>).required(true).help("Path to the parquet file"))
      .arg(arg!(--debug).required(false).help("Enable debug mode"))
      .arg(arg!(--sep <SEP>).required(false).default_value("\t").help("Field separator (default: tab)"))
      .arg(arg!(--nohdr).required(false).help("Do not print header in output"))
      .arg(arg!(--limit <LIMIT>).required(false).default_value("-1").help("limit output (default: none)"))
      .get_matches();

    Self {
      path: args.get_one::<String>("path").unwrap().to_string(),
      debug: args.get_flag("debug"),
      sep: args.get_one::<String>("sep").unwrap().to_string(),
      nohdr: args.get_flag("nohdr"),
      limit: args.get_one::<i32>("limit").unwrap().to_owned()
    }
  }
}