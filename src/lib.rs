pub const MAP_SIZE: usize = 256;

pub mod digraph;
pub mod hilbert;

pub enum Visualization {
    DigraphLinear,
    DigraphLog,
    Hilbert,
}
