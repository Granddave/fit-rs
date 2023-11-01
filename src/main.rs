use eframe::egui;
use egui::*;
use std::cmp::{max, min};

use anyhow::{anyhow, Context, Result};
use fit_rs::{digraph, hilbert, Visualization, MAP_SIZE};

// fn save_image(pixels: &[u8], filename: &str) -> Result<()> {
//     image::save_buffer(
//         filename,
//         &pixels,
//         MAP_SIZE as u32,
//         MAP_SIZE as u32,
//         image::ColorType::L8,
//     )
//     .context("save image")
// }

// fn main() -> Result<()> {
//     // TODO: use clap
//     let filepath = std::env::args().nth(1).context("read filename")?;
//     let bytes = std::fs::read(filepath).context("read file")?;
//
//     // Store the second cli argument as an int
//     let offset: usize = std::env::args()
//         .nth(2)
//         .unwrap_or("0".to_string())
//         .parse()
//         .context("parse offset")?;
//
//     if bytes.len() < 2 {
//         return Err(anyhow!("File is too short"));
//     }
//
//     let mut pixels = [0u8; MAP_SIZE * MAP_SIZE];
//
//     let slice_begin: usize = 0 + offset;
//     let slice_end: usize = min(
//         max(bytes.len(), MAP_SIZE * MAP_SIZE),
//         (MAP_SIZE * MAP_SIZE) + offset,
//     );
//
//     let vis = Visualization::Hilbert;
//     match vis {
//         Visualization::DigraphLinear => digraph::linear(&bytes, &mut pixels),
//         Visualization::DigraphLog => digraph::log(&bytes, &mut pixels),
//         Visualization::Hilbert => hilbert::linear(&bytes[slice_begin..slice_end], &mut pixels),
//     }
//
//     save_image(&pixels, &format!("image_{:0>6}.png", offset))?;
//
//     Ok(())
// }

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File to Image Transformer ",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

#[derive(Default)]
struct Content {
    size: usize,
    bytes: Vec<u8>,
    pixels: Vec<u8>, //u8; MAP_SIZE * MAP_SIZE],
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| i.key_pressed(Key::L)) {
                let filepath = std::env::args().nth(1).context("read filename").unwrap();
                self.bytes = std::fs::read(filepath).context("read file").unwrap();
                // Initialize pixels to 0 with the size of MAP_SIZE * MAP_SIZE
                self.pixels = vec![0; MAP_SIZE * MAP_SIZE];
                digraph::log(&self.bytes, &mut self.pixels)
            }

            if !self.pixels.is_empty() {
                for y in 0..MAP_SIZE {
                    for x in 0..MAP_SIZE {
                        let value = (self.pixels[y * MAP_SIZE + x]) as u8;
                        let s = self.size;
                        ui.painter().rect_filled(
                            Rect {
                                min: pos2((x * s) as f32, (y * s) as f32),
                                max: pos2(((x * s) + s) as f32, ((y * s) + s) as f32),
                            },
                            0.0,
                            Color32::from_gray(value),
                        );
                    }
                }
            }
            // if ui.button("Clear").clicked() {
            //     self.text.clear();
            // }

            if ctx.input(|i| i.key_pressed(Key::Q)) {
                self.size += 1;
            }
            if ctx.input(|i| i.key_pressed(Key::A)) {
                if self.size > 1 {
                    self.size -= 1;
                }
            }
        });
    }
}
