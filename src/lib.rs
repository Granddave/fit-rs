pub const MAP_SIZE: usize = 256;

pub mod digraph;
pub mod hilbert;

#[derive(Clone, Copy, Debug)]
pub enum Visualization {
    DigraphLinear,
    DigraphLog,
    Hilbert,
}

impl Default for Visualization {
    fn default() -> Self {
        Visualization::DigraphLog
    }
}
