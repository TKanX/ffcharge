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
    let scheme_key = match scheme {
        ProteinScheme::AmberFFSB => "amber-ffsb",
        ProteinScheme::AmberFF03 => "amber-ff03",
        ProteinScheme::Charmm => "charmm",
    };
    let pos_key = match pos {
        Position::NTerminal => "n",
        Position::NTerminalDeprotonated => "n-",
        Position::CTerminal => "c",
        Position::CTerminalProtonated => "c+",
        Position::FivePrime => "5",
        Position::ThreePrime => "3",
        Position::Middle => "m",
    };
    let atoms = generated::get_protein_atoms(scheme_key, pos_key, residue)?;
    Some(atoms.iter().map(|(_, c)| c).sum())
}

fn nucleic_total_charge(scheme: NucleicScheme, pos: Position, residue: &str) -> Option<f32> {
    let scheme_key = match scheme {
        NucleicScheme::Amber => "amber",
        NucleicScheme::Charmm => "charmm",
    };
    let pos_key = match pos {
        Position::NTerminal => "n",
        Position::NTerminalDeprotonated => "n-",
        Position::CTerminal => "c",
        Position::CTerminalProtonated => "c+",
        Position::FivePrime => "5",
        Position::ThreePrime => "3",
        Position::Middle => "m",
    };
    let atoms = generated::get_nucleic_atoms(scheme_key, pos_key, residue)?;
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

// =============================================================================
// Nucleic Acid Tests
// =============================================================================

mod nucleic {
    use super::*;

    macro_rules! test_nucleic_middle {
        ($scheme:ident, $residue:ident, $expected:expr) => {
            pastey::paste! {
                #[test]
                fn [<$scheme:lower _ $residue:lower _middle>]() {
                    let charge = nucleic_total_charge(
                        NucleicScheme::$scheme,
                        Position::Middle,
                        stringify!($residue),
                    ).expect(concat!("Missing: ", stringify!($scheme), "/m/", stringify!($residue)));
                    assert_charge_is_int(charge, $expected, concat!(stringify!($scheme), "/m/", stringify!($residue)));
                }
            }
        };
    }

    macro_rules! test_nucleic_terminal_sum {
        ($scheme:ident, $residue:ident, $expected:expr) => {
            pastey::paste! {
                #[test]
                fn [<$scheme:lower _ $residue:lower _terminal_sum>]() {
                    let five = nucleic_total_charge(NucleicScheme::$scheme, Position::FivePrime, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/5/", stringify!($residue)));
                    let three = nucleic_total_charge(NucleicScheme::$scheme, Position::ThreePrime, stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($scheme), "/3/", stringify!($residue)));
                    assert_charge_is_int(five + three, $expected, concat!(stringify!($scheme), " 5'+3' ", stringify!($residue)));
                }
            }
        };
    }

    macro_rules! test_nucleic_residue {
        ($scheme:ident, $residue:ident, $expected:expr) => {
            test_nucleic_middle!($scheme, $residue, $expected);
            test_nucleic_terminal_sum!($scheme, $residue, $expected);
        };
    }

    mod amber_dna {
        use super::*;
        test_nucleic_residue!(Amber, DA, -1);
        test_nucleic_residue!(Amber, DC, -1);
        test_nucleic_residue!(Amber, DG, -1);
        test_nucleic_residue!(Amber, DT, -1);
        test_nucleic_residue!(Amber, DI, -1);
    }

    mod amber_rna {
        use super::*;
        test_nucleic_residue!(Amber, A, -1);
        test_nucleic_residue!(Amber, C, -1);
        test_nucleic_residue!(Amber, G, -1);
        test_nucleic_residue!(Amber, U, -1);
        test_nucleic_residue!(Amber, I, -1);
    }

    mod charmm_dna {
        use super::*;
        test_nucleic_residue!(Charmm, DA, -1);
        test_nucleic_residue!(Charmm, DC, -1);
        test_nucleic_residue!(Charmm, DG, -1);
        test_nucleic_residue!(Charmm, DT, -1);
        test_nucleic_residue!(Charmm, DI, -1);
    }

    mod charmm_rna {
        use super::*;
        test_nucleic_residue!(Charmm, A, -1);
        test_nucleic_residue!(Charmm, C, -1);
        test_nucleic_residue!(Charmm, G, -1);
        test_nucleic_residue!(Charmm, U, -1);
        test_nucleic_residue!(Charmm, I, -1);
    }
}

// =============================================================================
// Water Tests
// =============================================================================

mod water {
    use super::*;

    macro_rules! test_water_model {
        ($scheme:ident) => {
            pastey::paste! {
                #[test]
                fn [<$scheme:lower _hydrogen_symmetry>]() {
                    let c = WaterScheme::$scheme.charges().expect(concat!("Missing: ", stringify!($scheme)));
                    assert_charge_eq(c.h1, c.h2, concat!(stringify!($scheme), " H1 == H2"));
                }

                #[test]
                fn [<$scheme:lower _total_charge>]() {
                    let c = WaterScheme::$scheme.charges().expect(concat!("Missing: ", stringify!($scheme)));
                    assert_charge_is_int(c.o + c.h1 + c.h2, 0, concat!(stringify!($scheme), " total"));
                }
            }
        };
    }

    test_water_model!(Tip3p);
    test_water_model!(Tip3pFb);
    test_water_model!(Spc);
    test_water_model!(SpcE);
    test_water_model!(Opc3);
}

// =============================================================================
// Ion Tests
// =============================================================================

mod ion {
    use super::*;

    macro_rules! test_ion {
        ($residue:ident, $expected:expr) => {
            pastey::paste! {
                #[test]
                fn [<classic_ $residue:lower>]() {
                    let charge = IonScheme::Classic.charge(stringify!($residue))
                        .expect(concat!("Missing: ", stringify!($residue)));
                    assert_charge_is_int(charge, $expected, stringify!($residue));
                }
            }
        };
        ($residue:literal, $expected:expr) => {
            pastey::paste! {
                #[test]
                fn [<classic_ $residue:lower>]() {
                    let charge = IonScheme::Classic.charge($residue)
                        .expect(concat!("Missing: ", $residue));
                    assert_charge_is_int(charge, $expected, $residue);
                }
            }
        };
    }

    // +1 Cations
    test_ion!(LI, 1);
    test_ion!(NA, 1);
    test_ion!(K, 1);
    test_ion!(RB, 1);
    test_ion!(CS, 1);
    test_ion!(AG, 1);
    test_ion!(AU, 1);
    test_ion!(CU1, 1);
    test_ion!(TL, 1);

    // +2 Cations
    test_ion!(MG, 2);
    test_ion!(CA, 2);
    test_ion!(SR, 2);
    test_ion!(BA, 2);
    test_ion!(FE2, 2);
    test_ion!(ZN, 2);
    test_ion!(CD, 2);
    test_ion!(HG, 2);
    test_ion!(CU, 2);
    test_ion!(NI, 2);
    test_ion!(CO, 2);
    test_ion!(MN, 2);
    test_ion!(PB, 2);
    test_ion!(PD, 2);
    test_ion!(PT, 2);
    test_ion!(EU, 2);

    // +3 Cations
    test_ion!(FE, 3);
    test_ion!(AL, 3);
    test_ion!(CR, 3);
    test_ion!(IN, 3);
    test_ion!(GA, 3);
    test_ion!(LA, 3);
    test_ion!(CE, 3);
    test_ion!(PR, 3);
    test_ion!(ND, 3);
    test_ion!(SM, 3);
    test_ion!(GD3, 3);
    test_ion!(TB, 3);
    test_ion!(DY, 3);
    test_ion!(ER3, 3);
    test_ion!(YB, 3);
    test_ion!(LU, 3);
    test_ion!(EU3, 3);
    test_ion!(HO3, 3);
    test_ion!(YT3, 3);
    test_ion!(RH3, 3);
    test_ion!(IR3, 3);
    test_ion!(AU3, 3);
    test_ion!(MN3, 3);
    test_ion!(SB, 3);
    test_ion!(V, 3);
    test_ion!(RU, 3);
    test_ion!(OS, 3);
    test_ion!("3CO", 3);
    test_ion!("3NI", 3);

    // +4 Cations
    test_ion!(ZR, 4);
    test_ion!(IR, 4);
    test_ion!(OS4, 4);
    test_ion!(PT4, 4);
    test_ion!(TH, 4);
    test_ion!("4MO", 4);

    // +6 Cations
    test_ion!(W, 6);
    test_ion!("6MO", 6);

    // -1 Anions
    test_ion!(F, -1);
    test_ion!(CL, -1);
    test_ion!(BR, -1);
    test_ion!(IOD, -1);
}
