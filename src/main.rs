use eframe::egui;
use egui::*;
use std::cmp::{max, min};

use fit_rs::{digraph, hilbert, Visualization, MAP_SIZE};

fn main() -> Result<(), eframe::Error> {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "File to Image Transformer ",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )
}

struct Content {
    zoom: usize,
    bytes: Vec<u8>,
    pixels: Vec<u8>,
    offset: usize,
    offset_step_size: usize,
    algoritm: Visualization,
    slice_begin: usize,
    slice_end: usize,
}

// Default for Content
impl Default for Content {
    fn default() -> Self {
        Content {
            zoom: 1,
            bytes: Vec::new(),
            pixels: Vec::new(),
            offset: 0,
            offset_step_size: 10,
            algoritm: Visualization::default(),
            slice_begin: 0,
            slice_end: 0,
        }
    }
}

impl Content {
    fn read_file(&mut self) {
        let filepath = std::env::args().nth(1).unwrap();
        self.bytes = std::fs::read(filepath).unwrap();
        self.pixels = vec![0; MAP_SIZE * MAP_SIZE];

        self.slice_end = min(
            max(self.bytes.len(), MAP_SIZE * MAP_SIZE),
            (MAP_SIZE * MAP_SIZE) + self.offset,
        );

        eprintln!("Pixels size: {}", self.pixels.len());
        self.calculate();
    }

    fn draw_map(&mut self, ui: &mut egui::Ui) {
        if !self.pixels.is_empty() {
            ui.painter().rect_filled(
                Rect {
                    min: pos2(0.0, 0.0),
                    max: pos2(MAP_SIZE as f32, MAP_SIZE as f32),
                },
                0.0,
                Color32::from_gray(0),
            );
            for y in 0..MAP_SIZE {
                for x in 0..MAP_SIZE {
                    let value = (self.pixels[y * MAP_SIZE + x]) as u8;
                    let s = self.zoom;
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
    }

    fn calculate(&mut self) {
        if self.bytes.len() < 2 {
            return;
        }
        eprintln!("Algoritm: {:?}", self.algoritm);
        match self.algoritm {
            Visualization::DigraphLinear => digraph::linear(&self.bytes, &mut self.pixels),
            Visualization::DigraphLog => digraph::log(&self.bytes, &mut self.pixels),
            Visualization::Hilbert => {
                self.slice_begin = 0 + self.offset;
                self.slice_end = min(
                    max(self.bytes.len(), MAP_SIZE * MAP_SIZE),
                    (MAP_SIZE * MAP_SIZE) + self.offset,
                );
                hilbert::linear(
                    &self.bytes[self.slice_begin..self.slice_end],
                    &mut self.pixels,
                )
            }
        }
    }
}

impl eframe::App for Content {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ctx.input(|i| i.key_pressed(Key::R)) {
                self.read_file();
            }

            self.draw_map(ui);

            if ctx.input(|i| i.key_pressed(Key::H)) {
                self.algoritm = Visualization::Hilbert;
                self.calculate();
            }
            if ctx.input(|i| i.key_pressed(Key::L)) {
                self.algoritm = Visualization::DigraphLog;
                self.calculate();
            }

            if ctx.input(|i| i.key_pressed(Key::ArrowRight)) {
                if self.offset + 10 * self.offset_step_size < self.bytes.len() {
                    self.offset += 10 * self.offset_step_size;
                    self.calculate();
                }
            }

            if ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
                if self.offset - self.offset_step_size > 0 {
                    self.offset -= 10 * self.offset_step_size;
                    self.calculate();
                }
            }

            if ctx.input(|i| i.key_pressed(Key::ArrowUp)) {
                self.offset_step_size += 10;
            }

            if ctx.input(|i| i.key_pressed(Key::ArrowDown)) {
                if self.offset_step_size != 0 {
                    self.offset_step_size -= 10;
                }
            }

            if ctx.input(|i| i.key_pressed(Key::Q)) {
                frame.close();
            }

            if ctx.input(|i| i.key_pressed(Key::PlusEquals)) {
                self.zoom += 1;
            }

            if ctx.input(|i| i.key_pressed(Key::Minus)) {
                if self.zoom > 1 {
                    self.zoom -= 1;
                }
            }
        });
    }
}
