use anyhow::{anyhow, Context, Result};

const MAP_SIZE: usize = 256;

mod hilbert {
    use super::MAP_SIZE;

    fn rot(n: i32, x: &mut i32, y: &mut i32, rx: i32, ry: i32) {
        if ry == 0 {
            if rx == 1 {
                *x = n - 1 - *x;
                *y = n - 1 - *y;
            }

            std::mem::swap(x, y);
        }
    }

    fn d2xy(total: i32, distance: i32) -> (i32, i32) {
        let mut x = 0;
        let mut y = 0;
        let mut t = distance;
        let mut s = 1;

        while s < total {
            let rx = 1 & (t / 2);
            let ry = 1 & (t ^ rx);
            rot(s, &mut x, &mut y, rx, ry);

            x += s * rx;
            y += s * ry;
            t /= 4;
            s *= 2;
        }

        (x, y)
    }

    pub fn linear(bytes: &[u8], pixels: &mut [u8]) {
        for (ix, byte) in bytes.iter().enumerate() {
            let (x, y) = d2xy(MAP_SIZE as i32, ix as i32);
            pixels[y as usize * MAP_SIZE + x as usize] = *byte;
        }
    }
}

mod digraph {
    use super::MAP_SIZE;
    pub fn linear(bytes: &[u8], pixels: &mut [u8]) {
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

    pub fn log(bytes: &[u8], pixels: &mut [u8]) {
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
                if value > max && value != 0 {
                    max = value.ilog2() as i32
                }
            }
        }

        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let value = map[y * MAP_SIZE + x];
                if value == 0 {
                    continue;
                }
                let value = (value.ilog2() * 255 / max as u32) as u8;
                let index: usize = y * MAP_SIZE + x;
                pixels[index] = value;
            }
        }
    }
}

enum Visualization {
    DigraphLinear,
    DigraphLog,
    Hilbert,
}

fn main() -> Result<()> {
    // TODO: use clap
    let filepath = std::env::args().nth(1).context("read filename")?;
    let bytes = std::fs::read(filepath).context("read file")?;

    if bytes.len() < 2 {
        return Err(anyhow!("File is too short"));
    }

    let mut pixels = [0u8; MAP_SIZE * MAP_SIZE];

    let vis = Visualization::DigraphLog;

    match vis {
        Visualization::DigraphLinear => digraph::linear(&bytes, &mut pixels),
        Visualization::DigraphLog => digraph::log(&bytes, &mut pixels),
        Visualization::Hilbert => hilbert::linear(&bytes, &mut pixels),
    }

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
