use ffcharge::{IonScheme, NucleicScheme, Position, ProteinScheme, WaterScheme};

// =============================================================================
// Generated Test Data
// =============================================================================

mod generated {
    include!(concat!(env!("OUT_DIR"), "/codegen_test.rs"));
}

// =============================================================================
// Test Utilities
// =============================================================================

const EPSILON: f32 = 1e-5;

fn assert_charge_eq(actual: f32, expected: f32, context: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff < EPSILON,
        "{}: expected {:.7}, got {:.7} (diff: {:.7})",
        context,
        expected,
        actual,
        diff
    );
}

fn assert_charge_is_int(actual: f32, expected: i32, context: &str) {
    let diff = (actual - expected as f32).abs();
    assert!(
        diff < EPSILON,
        "{}: expected {}, got {:.7} (diff: {:.7})",
        context,
        expected,
        actual,
        diff
    );
}

fn protein_total_charge(scheme: ProteinScheme, pos: Position, residue: &str) -> Option<f32> {
    let atoms = generated::get_protein_atoms(scheme.key(), pos.key(), residue)?;
    Some(atoms.iter().map(|(_, c)| c).sum())
}

fn nucleic_total_charge(scheme: NucleicScheme, pos: Position, residue: &str) -> Option<f32> {
    let atoms = generated::get_nucleic_atoms(scheme.key(), pos.key(), residue)?;
    Some(atoms.iter().map(|(_, c)| c).sum())
}
