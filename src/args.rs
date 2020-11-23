use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "args", about = "Arguments for PNGme.")]
pub enum Args {
    Encode {
        #[structopt(parse(from_os_str))]
        file_path: PathBuf,

        #[structopt()]
        chunk_type: String,

        #[structopt()]
        message: String,

        #[structopt(parse(from_os_str))]
        output_file: Option<PathBuf>,
    },

    Decode {
        #[structopt(parse(from_os_str))]
        file_path: PathBuf,

        #[structopt()]
        chunk_type: String,
    },

    Remove {
        #[structopt(parse(from_os_str))]
        file_path: PathBuf,

        #[structopt()]
        chunk_type: String,
    },

    Print {
        #[structopt(parse(from_os_str))]
        file_path: PathBuf,
    },
}
