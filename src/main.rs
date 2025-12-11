mod configuration;

extern crate clap;
extern crate polars;

use polars::prelude::*;

use configuration::Configuration;

fn main() {
  let conf = Configuration::new();

  if conf.debug {
    println!("Configuration: {:?}", conf);
  }

  let args = ScanArgsParquet::default();
  let path = PlPath::new(&conf.path);
  let df = LazyFrame::scan_parquet(path, args)
    .expect("Failed to read parquet file")
    .collect()
    .expect("Failed to collect DataFrame");

  let mut sep = "";

  if !conf.nohdr {
    for col in df.iter() {
      print!("{sep}{}", col.name());
      sep = &conf.sep;
    }
  }

  let mut limit = df.height(); 
  if conf.limit >= 0 && (conf.limit as usize) < df.height() {
    limit = conf.limit as usize;
  }

  for i in 0..limit {
    sep = "\n";
    for col in df.iter() {
      let val = col.get(i);
      print!("{sep}{}", val.unwrap());
      sep = &conf.sep;
    }
  }
}
