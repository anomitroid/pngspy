// src/commands/mod.rs

use crate::args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs, Cli, PngSpyArgs, EncodeUrlArgs};
use crate::Result;
use crate::png::Png;
use crate::chunks::chunk::Chunk;
use crate::chunks::chunk_type::ChunkType;
use std::str::FromStr;
use crate::network::downloader;

pub fn run(args: Cli) -> Result<()> {
    match args.command {
        PngSpyArgs::Encode(args) => handle_encode(args),
        PngSpyArgs::EncodeUrl(args) => handle_encode_url(args),
        PngSpyArgs::Decode(args) => handle_decode(args),
        PngSpyArgs::Remove(args) => handle_remove(args),
        PngSpyArgs::Print(args) => handle_print(args),
    }
}

pub fn handle_encode_url(args: EncodeUrlArgs) -> Result<()> {
    println!("Encode command invoked:");
    println!("  URL: {}", args.url);
    println!("  Chunk type: {}", args.chunk_type);
    println!("  Message: {}", args.message);

    let image_bytes = downloader::download_image(&args.url)?;
    let mut png = Png::try_from(image_bytes.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let data = args.message.into_bytes();
    let new_chunk = Chunk::new(chunk_type, data);
    png.append_chunk(new_chunk);
    let output_path = args.output.expect("Output path is required");
    png.save(&output_path)?;
    println!("PNG file encoded successfully and saved to {:?}", output_path);
    Ok(())
}

pub fn handle_encode(args: EncodeArgs) -> Result<()> {
    println!("Encode command invoked:");
    println!("  File path: {:?}", args.file_path);
    println!("  Chunk type: {}", args.chunk_type);
    println!("  Message: {}", args.message);
    println!("  Output: {:?}", args.output);
    
    let mut png = Png::from_file(&args.file_path)?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let data = args.message.into_bytes();
    let new_chunk = Chunk::new(chunk_type, data);
    png.append_chunk(new_chunk);
    let output_path = args.output.unwrap_or(args.file_path);
    png.save(&output_path)?;
    println!("PNG file encoded successfully and saved to {:?}", output_path);
    Ok(())
}

pub fn handle_decode(args: DecodeArgs) -> Result<()> {
    println!("Decode command invoked:");
    println!("  File path: {:?}", args.file_path);
    println!("  Chunk type: {}", args.chunk_type);

    let png = Png::from_file(&args.file_path)?;
    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("Chunk found:");
        println!("  Type: {}", chunk.chunk_type());
        println!("  Data: {}", String::from_utf8_lossy(&chunk.data()));
        let message = chunk.data_as_string()?;
        println!("  EncodedMessage: {}", message);
    } else {
        println!("No chunk found with type {}", args.chunk_type);
    }
    Ok(())
}

pub fn handle_remove(args: RemoveArgs) -> Result<()> {
    println!("Remove command invoked:");
    println!("  File path: {:?}", args.file_path);
    println!("  Chunk type: {}", args.chunk_type);

    let mut png = Png::from_file(&args.file_path)?;
    match png.remove_first_chunk(&args.chunk_type) {
        Ok(_) => {
            png.save(&args.file_path)?;
            println!("Chunk {} removed successfully", args.chunk_type);
        },
        Err(e) => {
            println!("Error removing chunk: {}", e);
        }
    }
    Ok(())
}

pub fn handle_print(args: PrintArgs) -> Result<()> {
    println!("Print command invoked:");
    println!("  File path: {:?}", args.file_path);

    let png = Png::from_file(&args.file_path)?;
    println!("{}", png);
    Ok(())
} 