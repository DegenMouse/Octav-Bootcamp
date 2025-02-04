use flate2::Compression;
use flate2::write::GzEncoder;
use std::env;
use std::fs::File;
use std::io::copy;
use std::io::BufReader;
use flate2::read::GzDecoder;

fn compress(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let mut input = BufReader::new(File::open(format!("src/{}", input_path))?);
    let output = File::create(format!("src/{}", output_path))?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut input, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

fn decompress(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input = BufReader::new(File::open(format!("src/{}", input_path))?);
    let mut output = File::create(format!("src/{}", output_path))?;
    let mut decoder = GzDecoder::new(input);
    copy(&mut decoder, &mut output)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    loop {
        println!("\nCompression Tool Instructions:");
        println!("1. To compress: program compress input_file output_file");
        println!("2. To decompress: program decompress input_file output_file");
        println!("3. Type 'exit' to quit");

        if args.len() != 4 {
            eprintln!("Usage: program [compress|decompress] input_file output_file");
            break;
        }

        let result = match args[1].as_str() {
            "compress" => compress(&args[2], &args[3]),
            "decompress" => decompress(&args[2], &args[3]),
            "exit" => break,
            _ => {
                eprintln!("Invalid command. Use 'compress' or 'decompress'");
                break;
            }
        };

        if let Err(e) = result {
            eprintln!("Error: {}", e);
            break;
        }

        println!("Operation completed successfully!");
        break;
    }
}
