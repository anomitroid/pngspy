use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use tempfile::NamedTempFile;

use pngspy::args::{
    DecodeArgs, EncodeArgs, EncodeUrlArgs, PrintArgs, RemoveArgs,
};
use pngspy::commands::{
    handle_decode, handle_encode, handle_encode_url, handle_print, handle_remove,
};
use pngspy::chunks::chunk_type::ChunkType;
use pngspy::png::Png;
use pngspy::Result;

fn create_minimal_png_file() -> NamedTempFile {
    let mut tmp = NamedTempFile::new().expect("Failed to create temporary file");
    tmp.write_all(&Png::STANDARD_HEADER)
        .expect("Failed to write PNG header");
    tmp
}

#[test]
fn test_encode_decode_remove_print() -> Result<()> {
    let tmp_png = create_minimal_png_file();
    let png_path = tmp_png.path().to_path_buf();

    let encode_args = EncodeArgs {
        file_path: png_path.clone(),
        chunk_type: "ruSt".to_string(),
        message: "secret message".to_string(),
        output: Some(png_path.clone()),
    };
    handle_encode(encode_args)?;

    let png = Png::from_file(&png_path)?;
    let chunk = png.chunk_by_type("ruSt");
    assert!(chunk.is_some(), "Chunk not found after encoding");
    assert_eq!(
        chunk.unwrap().data_as_string()?,
        "secret message",
        "Decoded message does not match encoded text"
    );

    let decode_args = DecodeArgs {
        file_path: png_path.clone(),
        chunk_type: "ruSt".to_string(),
    };
    handle_decode(decode_args)?;

    let remove_args = RemoveArgs {
        file_path: png_path.clone(),
        chunk_type: "ruSt".to_string(),
    };
    handle_remove(remove_args)?;

    let png_after_remove = Png::from_file(&png_path)?;
    let chunk_after_remove = png_after_remove.chunk_by_type("ruSt");
    assert!(
        chunk_after_remove.is_none(),
        "Chunk still present after removal"
    );

    let print_args = PrintArgs {
        file_path: png_path.clone(),
    };
    handle_print(print_args)?;
    Ok(())
}

#[test]
fn test_invalid_chunk_type() {
    let invalid_type = "abc";
    let res = ChunkType::from_str(invalid_type);
    assert!(
        res.is_err(),
        "ChunkType creation should fail for invalid input"
    );
}

#[test]
fn test_invalid_png_header() {
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(b"not a png header").unwrap();
    let res = Png::from_file(tmp.path());
    assert!(res.is_err(), "PNG creation should fail with an invalid header");
}

#[test]
// #[ignore]
fn test_encode_url_wikimedia_direct() -> Result<()> {
    let url = "https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png".to_string();
    let tmp_out = NamedTempFile::new().unwrap();
    let output_path: PathBuf = tmp_out.path().to_path_buf();

    let encode_url_args = EncodeUrlArgs {
        url,
        chunk_type: "teSt".to_string(),
        message: "url test message".to_string(),
        output: Some(output_path.clone()),
    };

    handle_encode_url(encode_url_args)?;
    let png = Png::from_file(&output_path)?;
    let chunk = png.chunk_by_type("teSt");
    assert!(chunk.is_some(), "Chunk not found in PNG from URL");
    assert_eq!(
        chunk.unwrap().data_as_string()?,
        "url test message",
        "Message does not match in PNG downloaded from URL"
    );
    Ok(())
}

#[test]
// #[ignore]
fn test_encode_url_wikipedia_gradient() -> Result<()> {
    let url = "https://upload.wikimedia.org/wikipedia/commons/1/17/PNG-Gradient_hex.png".to_string();
    let tmp_out = NamedTempFile::new().unwrap();
    let output_path: PathBuf = tmp_out.path().to_path_buf();

    let encode_url_args = EncodeUrlArgs {
        url,
        chunk_type: "Grad".to_string(),
        message: "Testing gradient".to_string(),
        output: Some(output_path.clone()),
    };

    handle_encode_url(encode_url_args)?;
    let png = Png::from_file(&output_path)?;
    let chunk = png.chunk_by_type("Grad");
    assert!(chunk.is_some(), "Chunk not found for gradient test");
    assert_eq!(
        chunk.unwrap().data_as_string()?,
        "Testing gradient",
        "Message does not match in gradient PNG"
    );
    Ok(())
}

#[test]
// #[ignore]
fn test_encode_url_wikipedia_color_depth() -> Result<()> {
    let url = "https://upload.wikimedia.org/wikipedia/commons/9/9d/PNG_color_depth_comparison.png".to_string();
    let tmp_out = NamedTempFile::new().unwrap();
    let output_path: PathBuf = tmp_out.path().to_path_buf();

    let encode_url_args = EncodeUrlArgs {
        url,
        chunk_type: "CdCp".to_string(),
        message: "Color depth test".to_string(),
        output: Some(output_path.clone()),
    };

    handle_encode_url(encode_url_args)?;
    let png = Png::from_file(&output_path)?;
    let chunk = png.chunk_by_type("CdCp");
    assert!(chunk.is_some(), "Chunk not found for color depth test");
    assert_eq!(
        chunk.unwrap().data_as_string()?,
        "Color depth test",
        "Message does not match in color depth PNG"
    );
    Ok(())
}

#[test]
fn test_remove_nonexistent_chunk() {
    let tmp_png = create_minimal_png_file();
    let png_path = tmp_png.path().to_path_buf();

    let remove_args = RemoveArgs {
        file_path: png_path,
        chunk_type: "NONE".to_string(),
    };

    let res = handle_remove(remove_args);
    assert!(res.is_ok(), "Removal should complete even if chunk doesn't exist");
}
