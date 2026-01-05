use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
struct Record {
    scheme: String,
    position: String,
    residue: String,
    atom: String,
    charge: f32,
}

#[derive(Debug, Default)]
struct WaterData {
    o: Option<f32>,
    h1: Option<f32>,
    h2: Option<f32>,
}

fn sanitize(s: &str) -> String {
    s.replace('-', "_").replace('+', "_plus")
}

fn main() {
    let path = Path::new("data/charges.csv");
    println!("cargo:rerun-if-changed={}", path.display());

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&dest_path).unwrap());

    let mut rdr = csv::ReaderBuilder::new()
        .comment(Some(b'#'))
        .trim(csv::Trim::All)
        .from_path(path)
        .expect("Failed to open data/charges.csv");

    let mut atom_data: HashMap<String, HashMap<String, HashMap<String, Vec<(String, f32)>>>> =
        HashMap::new();
    let mut ion_data: HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut water_data: HashMap<String, WaterData> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("Failed to parse CSV record");

        if record.residue == "HOH" {
            let entry = water_data.entry(record.scheme.clone()).or_default();
            match record.atom.as_str() {
                "O" => entry.o = Some(record.charge),
                "H1" => entry.h1 = Some(record.charge),
                "H2" => entry.h2 = Some(record.charge),
                _ => panic!("Unknown water atom: {}", record.atom),
            }
        } else if record.scheme == "classic" {
            let scheme_map = ion_data.entry(record.scheme.clone()).or_default();
            scheme_map.insert(record.residue, record.charge);
        } else {
            let scheme_map = atom_data.entry(record.scheme.clone()).or_default();
            let pos_map = scheme_map.entry(record.position.clone()).or_default();
            let res_vec = pos_map.entry(record.residue.clone()).or_default();
            res_vec.push((record.atom, record.charge));
        }
    }

    let mut match_arms_scheme = Vec::new();

    for (scheme, pos_map) in &atom_data {
        let scheme_ident = sanitize(scheme);
        let mut match_arms_pos = Vec::new();

        for (pos, res_map) in pos_map {
            let pos_ident = if pos.is_empty() {
                "empty".to_string()
            } else {
                sanitize(pos)
            };
            let map_name = format!(
                "MAP_{}_{}",
                scheme_ident.to_uppercase(),
                pos_ident.to_uppercase()
            );

            let mut entries = Vec::new();
            for (res, atoms) in res_map {
                let atoms_lit = atoms
                    .iter()
                    .map(|(a, c)| format!("(\"{}\", {:.7})", a, c))
                    .collect::<Vec<_>>()
                    .join(", ");
                let val = format!("&[{}]", atoms_lit);
                entries.push((res.clone(), val));
            }

            let mut phf_map = phf_codegen::Map::new();
            for (res, val) in &entries {
                phf_map.entry(res.as_str(), val.as_str());
            }

            writeln!(
                &mut file,
                "static {}: phf::Map<&'static str, &'static [(&'static str, f32)]> = {};",
                map_name,
                phf_map.build()
            )
            .unwrap();

            match_arms_pos.push(format!("\"{}\" => {}.get(res),", pos, map_name));
        }

        let pos_match = format!(
            "match pos {{\n            {}\n            _ => None,\n        }}",
            match_arms_pos.join("\n            ")
        );
        match_arms_scheme.push(format!(
            "\"{}\" => {{\n            {}\n        }}",
            scheme, pos_match
        ));
    }

    let mut match_arms_ion = Vec::new();
    for (scheme, res_map) in &ion_data {
        let scheme_ident = sanitize(scheme);
        let map_name = format!("ION_MAP_{}", scheme_ident.to_uppercase());

        let mut entries = Vec::new();
        for (res, charge) in res_map {
            entries.push((res.clone(), format!("{:.7}", charge)));
        }

        let mut phf_map = phf_codegen::Map::new();
        for (res, val) in &entries {
            phf_map.entry(res.as_str(), val.as_str());
        }

        writeln!(
            &mut file,
            "static {}: phf::Map<&'static str, f32> = {};",
            map_name,
            phf_map.build()
        )
        .unwrap();

        match_arms_ion.push(format!("\"{}\" => {}.get(res).copied(),", scheme, map_name));
    }

    let mut water_entries = Vec::new();
    for (scheme, data) in &water_data {
        let o = data.o.expect("Missing O");
        let h1 = data.h1.expect("Missing H1");
        let h2 = data.h2.expect("Missing H2");
        let val = format!(
            "crate::WaterCharges {{ o: {:.7}, h1: {:.7}, h2: {:.7} }}",
            o, h1, h2
        );
        water_entries.push((scheme.clone(), val));
    }

    let mut water_phf = phf_codegen::Map::new();
    for (scheme, val) in &water_entries {
        water_phf.entry(scheme.as_str(), val.as_str());
    }

    writeln!(
        &mut file,
        "static WATER_CHARGES: phf::Map<&'static str, crate::WaterCharges> = {};",
        water_phf.build()
    )
    .unwrap();

    writeln!(&mut file, "\n// Generated Lookup Functions").unwrap();

    let generate_lookup = |file: &mut BufWriter<File>, func_name: &str, allowed_pos: &[&str]| {
        let mut arms = Vec::new();
        for (scheme, pos_map) in &atom_data {
            let scheme_ident = sanitize(scheme);
            let mut pos_arms = Vec::new();
            let mut has_valid_pos = false;

            for (pos, _) in pos_map {
                if allowed_pos.contains(&pos.as_str()) {
                    let pos_ident = if pos.is_empty() {
                        "empty".to_string()
                    } else {
                        sanitize(pos)
                    };
                    let map_name = format!(
                        "MAP_{}_{}",
                        scheme_ident.to_uppercase(),
                        pos_ident.to_uppercase()
                    );
                    pos_arms.push(format!("\"{}\" => {}.get(res),", pos, map_name));
                    has_valid_pos = true;
                }
            }

            if has_valid_pos {
                let pos_match = format!(
                    "match pos {{\n            {}\n            _ => None,\n        }}",
                    pos_arms.join("\n            ")
                );
                arms.push(format!(
                    "\"{}\" => {{\n            {}\n        }}",
                    scheme, pos_match
                ));
            }
        }

        writeln!(file, "#[inline(always)]").unwrap();
        writeln!(
            file,
            "pub(crate) fn {}(scheme: &str, pos: &str, res: &str, atom: &str) -> Option<f32> {{",
            func_name
        )
        .unwrap();
        writeln!(file, "    let atoms = match scheme {{").unwrap();
        writeln!(file, "        {}", arms.join("\n        ")).unwrap();
        writeln!(file, "        _ => None,").unwrap();
        writeln!(file, "    }}?;").unwrap();
        writeln!(file, "    ").unwrap();
        writeln!(
            file,
            "    atoms.iter().find(|(a, _)| *a == atom).map(|(_, c)| *c)"
        )
        .unwrap();
        writeln!(file, "}}").unwrap();
    };

    generate_lookup(
        &mut file,
        "get_protein_charge",
        &["n", "n-", "c", "c+", "m"],
    );

    generate_lookup(&mut file, "get_nucleic_charge", &["5", "3", "m"]);

    writeln!(&mut file, "#[inline(always)]").unwrap();
    writeln!(
        &mut file,
        "pub(crate) fn get_ion_charge(scheme: &str, res: &str) -> Option<f32> {{"
    )
    .unwrap();
    writeln!(&mut file, "    match scheme {{").unwrap();
    writeln!(&mut file, "        {}", match_arms_ion.join("\n        ")).unwrap();
    writeln!(&mut file, "        _ => None,").unwrap();
    writeln!(&mut file, "    }}").unwrap();
    writeln!(&mut file, "}}").unwrap();

    writeln!(&mut file, "#[inline(always)]").unwrap();
    writeln!(
        &mut file,
        "pub(crate) fn get_water_charges(scheme: &str) -> Option<crate::WaterCharges> {{"
    )
    .unwrap();
    writeln!(&mut file, "    WATER_CHARGES.get(scheme).copied()").unwrap();
    writeln!(&mut file, "}}").unwrap();
}
