use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

pub fn encode(
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> io::Result<()> {
    // get Png from file
    let mut png = read_file(file_path.clone())?;

    // create Chunk to append to Png
    let chunk_type = ChunkType::from_str(chunk_type.as_str())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
    let data = message.bytes().collect();
    let chunk = Chunk::new(chunk_type, data);
    png.append_chunk(chunk);

    // write edited Png either to output_file or file_path
    if let Some(output_file) = output_file {
        write_file(output_file, png)
    } else {
        write_file(file_path, png)
    }
}

pub fn decode(file_path: PathBuf, chunk_type: String) -> io::Result<()> {
    let png = read_file(file_path)?;
    let chunk = png
        .chunk_by_type(chunk_type.as_str())
        .ok_or(io::Error::new(io::ErrorKind::NotFound, "chunk not found"))?;
    let message = chunk
        .data_as_string()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, ""))?;
    println!("decoded message: {}", message);
    Ok(())
}

pub fn remove(file_path: PathBuf, chunk_type: String) -> io::Result<()> {
    let mut png = read_file(file_path.clone())?;
    let removed_chunk = png
        .remove_chunk(chunk_type.as_str())
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;
    println!("removed chunk {}", removed_chunk);
    write_file(file_path, png)
}

pub fn print(file_path: PathBuf) -> io::Result<()> {
    let png = read_file(file_path)?;
    println!("{}", png);
    Ok(())
}

fn read_file(file_path: PathBuf) -> io::Result<Png> {
    // read file contents...
    let mut contents = Vec::new();
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    buf_reader.read_to_end(&mut contents)?;

    // ...and try to convert them to a Png
    Png::try_from(contents.as_ref()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn write_file(file_path: PathBuf, png: Png) -> io::Result<()> {
    let mut file = File::open(file_path)?;
    file.write_all(png.as_bytes().as_ref())?;
    Ok(())
}
