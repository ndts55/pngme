mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use crate::args::Args::{self, *};
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    let arguments = Args::from_args();
    match arguments {
        Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => commands::encode(file_path, chunk_type, message, output_file),
        Decode {
            file_path,
            chunk_type,
        } => commands::decode(file_path, chunk_type),
        Remove {
            file_path,
            chunk_type,
        } => commands::remove(file_path, chunk_type),
        Print { file_path } => commands::print(file_path),
    }
}
