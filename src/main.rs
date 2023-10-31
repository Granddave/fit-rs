use std::cmp::{max, min};

use anyhow::{anyhow, Context, Result};
use fit_rs::{digraph, hilbert, Visualization, MAP_SIZE};

fn save_image(pixels: &[u8], filename: &str) -> Result<()> {
    image::save_buffer(
        filename,
        &pixels,
        MAP_SIZE as u32,
        MAP_SIZE as u32,
        image::ColorType::L8,
    )
    .context("save image")
}

fn main() -> Result<()> {
    // TODO: use clap
    let filepath = std::env::args().nth(1).context("read filename")?;
    let bytes = std::fs::read(filepath).context("read file")?;

    // Store the second cli argument as an int
    let offset: usize = std::env::args()
        .nth(2)
        .unwrap_or("0".to_string())
        .parse()
        .context("parse offset")?;

    if bytes.len() < 2 {
        return Err(anyhow!("File is too short"));
    }

    let mut pixels = [0u8; MAP_SIZE * MAP_SIZE];

    let slice_begin: usize = 0 + offset;
    let slice_end: usize = min(
        max(bytes.len(), MAP_SIZE * MAP_SIZE),
        (MAP_SIZE * MAP_SIZE) + offset,
    );

    let vis = Visualization::Hilbert;
    match vis {
        Visualization::DigraphLinear => digraph::linear(&bytes, &mut pixels),
        Visualization::DigraphLog => digraph::log(&bytes, &mut pixels),
        Visualization::Hilbert => hilbert::linear(&bytes[slice_begin..slice_end], &mut pixels),
    }

    save_image(&pixels, &format!("image_{:0>6}.png", offset))?;

    Ok(())
}
