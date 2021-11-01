use std::error::Error;
use structopt::StructOpt;

mod opt;
use opt::Opt;
mod err;
mod core;
use self::core::read::{load_csv, write_csv};
use std::path::PathBuf;
use std::process;

fn main() {
    let opt = Opt::from_args();
    let filename = PathBuf::from(opt.input);
    let csv_data = match load_csv(filename) {
        Ok(fname) => {fname}
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1)
        }
    };
    let output_file = &opt.output.unwarp_or("output/output.csv".to_string());
    match write_csv(&csv_data, output_file) {
        Ok(_) => {
            println!("write success")
        }
        Err(e) => {
            println!("main error: {:?}", e);
            process::exit(1)
        }
    }
    println!("{:?}",opt);
}
