//! A lightweight pure Rust library for fast, residue-based force field charge assignment (AMBER/CHARMM)
//! in molecular modeling pipelines.
//!
//! # FFCharge
//!
//! **FFCharge** provides pre-computed atomic partial charges from AMBER and CHARMM force
//! fields for proteins, nucleic acids, water, and ions. Designed with **`no_std`**
//! support and zero runtime dependencies, it is ideal for high-performance molecular
//! dynamics preprocessing and biomolecular structure analysis.
//!
//! ## Features
//!
//! - **O(1) Lookups**: Uses compile-time perfect hash functions (PHF) for constant-time
//!   charge retrieval.
//! - **Comprehensive Coverage**: Supports 29 protein residues, 10 nucleic acid residues,
//!   5 water models, and 66 ion types.
//! - **Terminal-Aware**: Handles N-/C-terminal protein residues (including protonation
//!   variants) and 5'/3'-terminal nucleic acids.
//! - **Type-Safe API**: Strongly-typed enums for schemes and positions prevent invalid
//!   queries at compile time.
//! - **`no_std` Compatible**: Suitable for embedded systems and WebAssembly targets.
//!
//! ## Quick Start
//!
//! ```rust
//! use ffcharge::{ProteinScheme, NucleicScheme, WaterScheme, IonScheme, Position};
//!
//! // Protein: Get charge for CA atom of Alanine (middle position, AMBER ff99SB)
//! let charge = ProteinScheme::AmberFFSB
//!     .charge(Position::Middle, "ALA", "CA")
//!     .expect("Charge not found");
//! assert!((charge - 0.0337).abs() < 1e-4);
//!
//! // Protein: N-terminal residue
//! let n_term = ProteinScheme::AmberFFSB
//!     .charge(Position::NTerminal, "ALA", "N")
//!     .expect("Charge not found");
//! assert!((n_term - 0.1414).abs() < 1e-4);
//!
//! // Nucleic acid: DNA adenine at 5' terminus (AMBER)
//! let dna = NucleicScheme::Amber
//!     .charge(Position::FivePrime, "DA", "N9")
//!     .expect("Charge not found");
//! assert!((dna - (-0.0268)).abs() < 1e-4);
//!
//! // Water: TIP3P model
//! let water = WaterScheme::Tip3p.charges().expect("Water model not found");
//! assert!((water.o - (-0.834)).abs() < 1e-3);
//! assert!((water.h1 - 0.417).abs() < 1e-3);
//!
//! // Ion: Sodium
//! let na = IonScheme::Classic.charge("NA").expect("Ion not found");
//! assert!((na - 1.0).abs() < 1e-6);
//! ```
//!
//! ## Supported Force Fields
//!
//! | Category | Schemes |
//! |----------|---------|
//! | Protein  | AMBER ff99SB/ff14SB/ff19SB, AMBER ff03, CHARMM C22/C27/C36/C36m |
//! | Nucleic  | AMBER OL15/OL21/OL24/bsc1/OL3, CHARMM C27/C36 |
//! | Water    | TIP3P, TIP3P-FB, SPC, SPC/E, OPC3 |
//! | Ion      | Classic (formal charges) |
//!
//! ## Position Handling
//!
//! Residues at chain termini have different charge distributions. Use [`Position`] to
//! specify the residue location:
//!
//! | Position | Description | Applicable To |
//! |----------|-------------|---------------|
//! | `NTerminal` | N-terminal (protonated NH₃⁺) | Protein |
//! | `NTerminalDeprotonated` | N-terminal (neutral NH₂) | Protein |
//! | `CTerminal` | C-terminal (deprotonated COO⁻) | Protein |
//! | `CTerminalProtonated` | C-terminal (protonated COOH) | Protein |
//! | `FivePrime` | 5'-terminal | Nucleic acid |
//! | `ThreePrime` | 3'-terminal | Nucleic acid |
//! | `Middle` | Internal residue (default) | Both |

#![no_std]

mod position;
mod scheme;

pub use position::Position;
pub use scheme::{IonScheme, NucleicScheme, ProteinScheme, WaterScheme};

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
