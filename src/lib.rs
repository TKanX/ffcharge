#![no_std]

mod position;
mod scheme;

pub use position::Position;
pub use scheme::{DnaScheme, IonScheme, ProteinScheme, RnaScheme, WaterScheme};

/// Water charge distribution.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterCharges {
    /// Charge on Oxygen.
    pub o: f32,
    /// Charge on Hydrogen 1.
    pub h1: f32,
    /// Charge on Hydrogen 2.
    pub h2: f32,
}

mod generated {
    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));
}
