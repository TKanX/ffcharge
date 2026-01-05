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

// =============================================================================
// Protein Tests
// =============================================================================

mod protein {
    use super::*;

    macro_rules! test_protein_middle {
        ($scheme:ident, $residue:ident, $expected:expr) => {
            pastey::paste! {
                #[test]
                fn [<$scheme:lower _ $residue:lower _middle>]() {
                    let charge = protein_total_charge(
                        ProteinScheme::$scheme,
                        Position::Middle,
                        stringify!($residue),
                    ).expect(concat!("Missing: ", stringify!($scheme), "/m/", stringify!($residue)));
                    assert_charge_is_int(charge, $expected, concat!(stringify!($scheme), "/m/", stringify!($residue)));
                }
            }
        };
    }

    macro_rules! test_protein_terminals {
        ($scheme:ident, $residue:ident) => {
            pastey::paste! {
                #[test]
                fn [<$scheme:lower _ $residue:lower _n_terminal>]() {
                    let n = protein_total_charge(ProteinScheme::$scheme, Position::NTerminal, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/n/", stringify!($residue)));
                    let m = protein_total_charge(ProteinScheme::$scheme, Position::Middle, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/m/", stringify!($residue)));
                    assert_charge_is_int(n, (m + 1.0).round() as i32, concat!(stringify!($scheme), "/n/", stringify!($residue)));
                }

                #[test]
                fn [<$scheme:lower _ $residue:lower _c_terminal>]() {
                    let c = protein_total_charge(ProteinScheme::$scheme, Position::CTerminal, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/c/", stringify!($residue)));
                    let m = protein_total_charge(ProteinScheme::$scheme, Position::Middle, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/m/", stringify!($residue)));
                    assert_charge_is_int(c, (m - 1.0).round() as i32, concat!(stringify!($scheme), "/c/", stringify!($residue)));
                }

                #[test]
                fn [<$scheme:lower _ $residue:lower _n_terminal_deprotonated>]() {
                    let nd = protein_total_charge(ProteinScheme::$scheme, Position::NTerminalDeprotonated, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/n-/", stringify!($residue)));
                    let m = protein_total_charge(ProteinScheme::$scheme, Position::Middle, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/m/", stringify!($residue)));
                    assert_charge_is_int(nd, m.round() as i32, concat!(stringify!($scheme), "/n-/", stringify!($residue)));
                }

                #[test]
                fn [<$scheme:lower _ $residue:lower _c_terminal_protonated>]() {
                    let cp = protein_total_charge(ProteinScheme::$scheme, Position::CTerminalProtonated, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/c+/", stringify!($residue)));
                    let m = protein_total_charge(ProteinScheme::$scheme, Position::Middle, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/m/", stringify!($residue)));
                    assert_charge_is_int(cp, m.round() as i32, concat!(stringify!($scheme), "/c+/", stringify!($residue)));
                }
            }
        };
    }

    macro_rules! test_protein_residue {
        ($scheme:ident, $residue:ident, $expected:expr) => {
            test_protein_middle!($scheme, $residue, $expected);
            test_protein_terminals!($scheme, $residue);
        };
    }

    mod amber_ffsb {
        use super::*;
        test_protein_residue!(AmberFFSB, ALA, 0);
        test_protein_residue!(AmberFFSB, ASN, 0);
        test_protein_residue!(AmberFFSB, CYS, 0);
        test_protein_residue!(AmberFFSB, CYX, 0);
        test_protein_residue!(AmberFFSB, GLN, 0);
        test_protein_residue!(AmberFFSB, GLY, 0);
        test_protein_residue!(AmberFFSB, HID, 0);
        test_protein_residue!(AmberFFSB, HIE, 0);
        test_protein_residue!(AmberFFSB, ILE, 0);
        test_protein_residue!(AmberFFSB, LEU, 0);
        test_protein_residue!(AmberFFSB, MET, 0);
        test_protein_residue!(AmberFFSB, PHE, 0);
        test_protein_residue!(AmberFFSB, PRO, 0);
        test_protein_residue!(AmberFFSB, SER, 0);
        test_protein_residue!(AmberFFSB, THR, 0);
        test_protein_residue!(AmberFFSB, TRP, 0);
        test_protein_residue!(AmberFFSB, TYR, 0);
        test_protein_residue!(AmberFFSB, VAL, 0);
        test_protein_residue!(AmberFFSB, ASH, 0);
        test_protein_residue!(AmberFFSB, GLH, 0);
        test_protein_residue!(AmberFFSB, ARN, 0);
        test_protein_residue!(AmberFFSB, LYN, 0);
        test_protein_residue!(AmberFFSB, ARG, 1);
        test_protein_residue!(AmberFFSB, HIP, 1);
        test_protein_residue!(AmberFFSB, LYS, 1);
        test_protein_residue!(AmberFFSB, ASP, -1);
        test_protein_residue!(AmberFFSB, GLU, -1);
        test_protein_residue!(AmberFFSB, CYM, -1);
        test_protein_residue!(AmberFFSB, TYM, -1);
    }

    mod amber_ff03 {
        use super::*;
        test_protein_residue!(AmberFF03, ALA, 0);
        test_protein_residue!(AmberFF03, ASN, 0);
        test_protein_residue!(AmberFF03, CYS, 0);
        test_protein_residue!(AmberFF03, CYX, 0);
        test_protein_residue!(AmberFF03, GLN, 0);
        test_protein_residue!(AmberFF03, GLY, 0);
        test_protein_residue!(AmberFF03, HID, 0);
        test_protein_residue!(AmberFF03, HIE, 0);
        test_protein_residue!(AmberFF03, ILE, 0);
        test_protein_residue!(AmberFF03, LEU, 0);
        test_protein_residue!(AmberFF03, MET, 0);
        test_protein_residue!(AmberFF03, PHE, 0);
        test_protein_residue!(AmberFF03, PRO, 0);
        test_protein_residue!(AmberFF03, SER, 0);
        test_protein_residue!(AmberFF03, THR, 0);
        test_protein_residue!(AmberFF03, TRP, 0);
        test_protein_residue!(AmberFF03, TYR, 0);
        test_protein_residue!(AmberFF03, VAL, 0);
        test_protein_residue!(AmberFF03, ASH, 0);
        test_protein_residue!(AmberFF03, GLH, 0);
        test_protein_residue!(AmberFF03, ARN, 0);
        test_protein_residue!(AmberFF03, LYN, 0);
        test_protein_residue!(AmberFF03, ARG, 1);
        test_protein_residue!(AmberFF03, HIP, 1);
        test_protein_residue!(AmberFF03, LYS, 1);
        test_protein_residue!(AmberFF03, ASP, -1);
        test_protein_residue!(AmberFF03, GLU, -1);
        test_protein_residue!(AmberFF03, CYM, -1);
        test_protein_residue!(AmberFF03, TYM, -1);
    }

    mod charmm {
        use super::*;
        test_protein_residue!(Charmm, ALA, 0);
        test_protein_residue!(Charmm, ASN, 0);
        test_protein_residue!(Charmm, CYS, 0);
        test_protein_residue!(Charmm, CYX, 0);
        test_protein_residue!(Charmm, GLN, 0);
        test_protein_residue!(Charmm, GLY, 0);
        test_protein_residue!(Charmm, HID, 0);
        test_protein_residue!(Charmm, HIE, 0);
        test_protein_residue!(Charmm, ILE, 0);
        test_protein_residue!(Charmm, LEU, 0);
        test_protein_residue!(Charmm, MET, 0);
        test_protein_residue!(Charmm, PHE, 0);
        test_protein_residue!(Charmm, PRO, 0);
        test_protein_residue!(Charmm, SER, 0);
        test_protein_residue!(Charmm, THR, 0);
        test_protein_residue!(Charmm, TRP, 0);
        test_protein_residue!(Charmm, TYR, 0);
        test_protein_residue!(Charmm, VAL, 0);
        test_protein_residue!(Charmm, ASH, 0);
        test_protein_residue!(Charmm, GLH, 0);
        test_protein_residue!(Charmm, ARN, 0);
        test_protein_residue!(Charmm, LYN, 0);
        test_protein_residue!(Charmm, ARG, 1);
        test_protein_residue!(Charmm, HIP, 1);
        test_protein_residue!(Charmm, LYS, 1);
        test_protein_residue!(Charmm, ASP, -1);
        test_protein_residue!(Charmm, GLU, -1);
        test_protein_residue!(Charmm, CYM, -1);
        test_protein_residue!(Charmm, TYM, -1);
    }
}
