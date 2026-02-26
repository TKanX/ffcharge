# FFCharge

**FFCharge** is a lightweight, pure Rust library for fast, residue-based force field partial charge assignment in molecular modeling pipelines. It provides pre-computed atomic partial charges from AMBER and CHARMM force fields for proteins, nucleic acids, water, and ions.

Designed with **`no_std`** support and **zero runtime dependencies**, FFCharge is ideal for high-performance molecular dynamics preprocessing, biomolecular structure analysis, and integration into existing simulation workflows.

## Features

- **Zero Runtime Dependencies**: Uses compile-time code generation with [PHF](https://crates.io/crates/phf) for O(1) lookups.
- **`no_std` Compatible**: Suitable for embedded systems and WebAssembly targets.
- **Comprehensive Coverage**: Supports **29 protein residues**, **10 nucleic acid residues**, **5 water models**, and **66 ion types**.
- **Multiple Force Fields**: AMBER (ff99SB/ff14SB/ff19SB, ff03) and CHARMM (C22/C27/C36/C36m) for proteins; AMBER and CHARMM for nucleic acids.
- **Terminal-Aware**: Handles N-terminal, C-terminal, and their protonation variants for proteins; 5' and 3' termini for nucleic acids.
- **Type-Safe API**: Strongly-typed enums for schemes and positions prevent runtime errors.

## Installation

Add FFCharge to your `Cargo.toml`:

```toml
[dependencies]
ffcharge = "0.2.1"
```

## Quick Start

```rust
use ffcharge::{ProteinScheme, NucleicScheme, WaterScheme, IonScheme, Position};

fn main() {
    // Protein: Get charge for CA atom of Alanine (middle position, AMBER ff99SB)
    let charge = ProteinScheme::AmberFFSB
        .charge(Position::Middle, "ALA", "CA")
        .expect("Charge not found");
    println!("ALA CA charge: {:.4}", charge);

    // Protein: N-terminal residue
    let n_term_charge = ProteinScheme::AmberFFSB
        .charge(Position::NTerminal, "ALA", "N")
        .expect("Charge not found");
    println!("N-terminal ALA N charge: {:.4}", n_term_charge);

    // Nucleic acid: DNA adenine at 5' terminus (AMBER)
    let dna_charge = NucleicScheme::Amber
        .charge(Position::FivePrime, "DA", "N9")
        .expect("Charge not found");
    println!("DA N9 charge: {:.4}", dna_charge);

    // Water: TIP3P model
    let water = WaterScheme::Tip3p.charges().expect("Water model not found");
    println!("TIP3P: O={:.4}, H1={:.4}, H2={:.4}", water.o, water.h1, water.h2);

    // Ion: Sodium
    let na_charge = IonScheme::Classic
        .charge("NA")
        .expect("Ion not found");
    println!("Na+ charge: {:.1}", na_charge);
}
```

## API Reference

For detailed API documentation, visit the [API Documentation](https://docs.rs/ffcharge).

## Data Coverage

For complete residue and atom listings, see [data/README.md](data/README.md).

**Summary:**

- **Proteins**: **8,169** charge entries (29 residues × 5 positions × 3 schemes, varying atoms per residue)
- **Nucleic Acids**: **1,321** charge entries (10 residues × 3 positions × 2 schemes, varying atoms per residue)
- **Water**: **15** charge entries (5 models × 3 atoms)
- **Ions**: **66** ion types (formal charges)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
