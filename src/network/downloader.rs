// src/network/downloader.rs

use crate::error::Result;

pub fn download_image(url: &str) -> Result<Vec<u8>> {
    // Download the image using reqwest's blocking API.
    let response = reqwest::blocking::get(url)?;
    let bytes = response.bytes()?;
    Ok(bytes.to_vec())
}