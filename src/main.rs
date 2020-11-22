mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::Args;
use structopt::StructOpt;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let arguments = Args::from_args();
    println!("{:?}", arguments);
}
