use anyhow::{anyhow, Context, Result};

const MAP_SIZE: usize = 256;

fn digraph_linear(bytes: &[u8], pixels: &mut [u8]) {
    let mut map = [0i32; MAP_SIZE * MAP_SIZE];

    for i in 0..bytes.len() - 1 {
        let x: u8 = bytes[i];
        let y: u8 = bytes[i + 1];
        map[(x as usize) * MAP_SIZE + y as usize] += 1;
    }
    let mut max = 0;
    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let value = map[y * MAP_SIZE + x];
            if value > max {
                max = value;
            }
        }
    }

    for y in 0..MAP_SIZE {
        for x in 0..MAP_SIZE {
            let value = (map[y * MAP_SIZE + x] * 255 / max) as u8;
            let index: usize = y * MAP_SIZE + x;
            pixels[index] = value;
        }
    }
}

fn main() -> Result<()> {
    // TODO: use clap
    let filepath = std::env::args().nth(1).context("read filename")?;
    let bytes = std::fs::read(filepath).context("read file")?;

    if bytes.len() < 2 {
        return Err(anyhow!("File is too short"));
    }

    let mut pixels = [0u8; MAP_SIZE * MAP_SIZE];
    digraph_linear(&bytes, &mut pixels);
    image::save_buffer(
        "image.png",
        &pixels,
        MAP_SIZE as u32,
        MAP_SIZE as u32,
        image::ColorType::L8,
    )
    .context("save image")?;

    Ok(())
}
