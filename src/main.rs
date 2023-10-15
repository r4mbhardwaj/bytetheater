extern crate flate2;

use flate2::{bufread::GzDecoder, write::GzEncoder, Compression};
use std::fs::File;
use std::io::{copy, BufReader, BufWriter};

pub struct ByteTheater {}

const EXTENSION: &str = ".gz";

impl ByteTheater {
    pub fn compress(movie_path: &str, compressed_path: &str) -> std::io::Result<()> {
        let movie_file = File::open(movie_path)?;
        let mut reader = BufReader::new(movie_file);

        let compressed_file = File::create(compressed_path)?;
        let encoder = GzEncoder::new(compressed_file, Compression::default());

        let mut writer = BufWriter::new(encoder);
        copy(&mut reader, &mut writer)?;

        Ok(())
    }

    pub fn decompress(compressed_path: &str, decompressed_path: &str) -> std::io::Result<()> {
        let compressed_file = File::open(compressed_path)?;
        let reader = BufReader::new(compressed_file);
        let mut decoder = GzDecoder::new(reader);

        let decompressed_file = File::create(decompressed_path)?;
        let mut writer = BufWriter::new(decompressed_file);
        copy(&mut decoder, &mut writer)?;

        Ok(())
    }
}

fn main() {
    println!("ByteTheater: A video compression tool written in Rust.");

    let args = std::env::args().collect::<Vec<String>>();
    println!("args: {:?}", args);

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let mode = args[1].clone();
    let filename = args[2].clone();

    match mode.as_str() {
        "dump" => {
            println!("Compressing {} to {}{} ...", filename, filename, EXTENSION);
            ByteTheater::compress(&filename, &format!("{}{}", filename, EXTENSION)).unwrap();
        }
        "load" => {
            let filename = filename.trim_end_matches(EXTENSION);
            println!(
                "Decompressing {}{} to {} ...",
                filename, EXTENSION, filename
            );
            println!("{}{}", filename, EXTENSION);
            println!("{}", filename);

            let folder_path = filename.split("/").collect::<Vec<&str>>();
            let exact_file_name = format!("Output_{}", folder_path[&folder_path.len() - 1]);

            let output_path = format!("{}/{}", folder_path[0], exact_file_name);
            println!("output_path: {}", output_path);
            ByteTheater::decompress(
                &format!("{}{}", filename, EXTENSION),
                &format!("{}", output_path),
            )
            .unwrap();
        }
        _ => {
            println!("Usage: {} <filename>", args[0]);
            std::process::exit(1);
        }
    }

    /* Streaming method usage ... */
}
