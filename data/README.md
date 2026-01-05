# FFCharge Data

This directory contains the charge data used by FFCharge.

## File Format

The `charges.csv` file uses CSV format with the following columns:

| Column     | Description                                    |
| ---------- | ---------------------------------------------- |
| `scheme`   | Force field scheme identifier                  |
| `position` | Residue position in chain                      |
| `residue`  | Residue name (3-letter code)                   |
| `atom`     | Atom name                                      |
| `charge`   | Partial charge value (elementary charge units) |

## Data Coverage

| Category     | Entries | Residues | Schemes |
| ------------ | ------- | -------- | ------- |
| Protein      | 8,169   | 29       | 3       |
| Nucleic Acid | 1,321   | 10       | 2       |
| Water        | 15      | 1        | 5       |
| Ion          | 66      | 66       | 1       |

---

## Protein Residues

### Standard Amino Acids

| Residue | Description                   | Net Charge |
| ------- | ----------------------------- | ---------- |
| ALA     | Alanine                       | 0          |
| ARG     | Arginine (protonated)         | +1         |
| ASN     | Asparagine                    | 0          |
| ASP     | Aspartate (deprotonated)      | −1         |
| CYS     | Cysteine                      | 0          |
| CYX     | Cystine (disulfide-bonded)    | 0          |
| GLN     | Glutamine                     | 0          |
| GLU     | Glutamate (deprotonated)      | −1         |
| GLY     | Glycine                       | 0          |
| HID     | Histidine (δ-protonated)      | 0          |
| HIE     | Histidine (ε-protonated)      | 0          |
| HIP     | Histidine (doubly protonated) | +1         |
| ILE     | Isoleucine                    | 0          |
| LEU     | Leucine                       | 0          |
| LYS     | Lysine (protonated)           | +1         |
| MET     | Methionine                    | 0          |
| PHE     | Phenylalanine                 | 0          |
| PRO     | Proline                       | 0          |
| SER     | Serine                        | 0          |
| THR     | Threonine                     | 0          |
| TRP     | Tryptophan                    | 0          |
| TYR     | Tyrosine                      | 0          |
| VAL     | Valine                        | 0          |

### Non-Standard/Protonation Variants

| Residue | Description               | Net Charge |
| ------- | ------------------------- | ---------- |
| ARN     | Arginine (neutral)        | 0          |
| ASH     | Aspartate (protonated)    | 0          |
| CYM     | Cysteine (deprotonated)   | −1         |
| GLH     | Glutamate (protonated)    | 0          |
| LYN     | Lysine (neutral)          | 0          |
| TYM     | Tyrosinate (deprotonated) | −1         |

### Protein Atom Names by Residue and Position

> **Position Key**: `n` = N-terminal, `n-` = N-terminal deprotonated, `m` = Middle, `c` = C-terminal, `c+` = C-terminal protonated

| Residue | Position | Atoms                                                                                                                 |
| ------- | -------- | --------------------------------------------------------------------------------------------------------------------- |
| ALA     | n        | N, H1, H2, H3, CA, HA, C, O, CB, HB1, HB2, HB3                                                                        |
| ALA     | n-       | N, H1, H2, CA, HA, C, O, CB, HB1, HB2, HB3                                                                            |
| ALA     | m        | N, H, CA, HA, C, O, CB, HB1, HB2, HB3                                                                                 |
| ALA     | c        | N, H, CA, HA, C, O, OXT, CB, HB1, HB2, HB3                                                                            |
| ALA     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, HB1, HB2, HB3                                                                      |
| ARG     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21, HH22   |
| ARG     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21, HH22       |
| ARG     | m        | N, H, CA, HA, C, O, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21, HH22            |
| ARG     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21, HH22       |
| ARG     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21, HH22 |
| ARN     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21         |
| ARN     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21             |
| ARN     | m        | N, H, CA, HA, C, O, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21                  |
| ARN     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21             |
| ARN     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CG, CZ, NE, NH1, NH2, HB2, HB3, HD2, HD3, HE, HG2, HG3, HH11, HH12, HH21       |
| ASH     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CG, OD1, OD2, HB2, HB3, HD2                                                          |
| ASH     | n-       | N, H1, H2, CA, HA, C, O, CB, CG, OD1, OD2, HB2, HB3, HD2                                                              |
| ASH     | m        | N, H, CA, HA, C, O, CB, CG, OD1, OD2, HB2, HB3, HD2                                                                   |
| ASH     | c        | N, H, CA, HA, C, O, OXT, CB, CG, OD1, OD2, HB2, HB3, HD2                                                              |
| ASH     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CG, OD1, OD2, HB2, HB3, HD2                                                        |
| ASN     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CG, ND2, OD1, HB2, HB3, HD21, HD22                                                   |
| ASN     | n-       | N, H1, H2, CA, HA, C, O, CB, CG, ND2, OD1, HB2, HB3, HD21, HD22                                                       |
| ASN     | m        | N, H, CA, HA, C, O, CB, CG, ND2, OD1, HB2, HB3, HD21, HD22                                                            |
| ASN     | c        | N, H, CA, HA, C, O, OXT, CB, CG, ND2, OD1, HB2, HB3, HD21, HD22                                                       |
| ASN     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CG, ND2, OD1, HB2, HB3, HD21, HD22                                                 |
| ASP     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CG, OD1, OD2, HB2, HB3                                                               |
| ASP     | n-       | N, H1, H2, CA, HA, C, O, CB, CG, OD1, OD2, HB2, HB3                                                                   |
| ASP     | m        | N, H, CA, HA, C, O, CB, CG, OD1, OD2, HB2, HB3                                                                        |
| ASP     | c        | N, H, CA, HA, C, O, OXT, CB, CG, OD1, OD2, HB2, HB3                                                                   |
| ASP     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CG, OD1, OD2, HB2, HB3                                                             |
| CYM     | n        | N, H1, H2, H3, CA, HA, C, O, CB, SG, HB2, HB3                                                                         |
| CYM     | n-       | N, H1, H2, CA, HA, C, O, CB, SG, HB2, HB3                                                                             |
| CYM     | m        | N, H, CA, HA, C, O, CB, SG, HB2, HB3                                                                                  |
| CYM     | c        | N, H, CA, HA, C, O, OXT, CB, SG, HB2, HB3                                                                             |
| CYM     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, SG, HB2, HB3                                                                       |
| CYS     | n        | N, H1, H2, H3, CA, HA, C, O, CB, SG, HB2, HB3, HG                                                                     |
| CYS     | n-       | N, H1, H2, CA, HA, C, O, CB, SG, HB2, HB3, HG                                                                         |
| CYS     | m        | N, H, CA, HA, C, O, CB, SG, HB2, HB3, HG                                                                              |
| CYS     | c        | N, H, CA, HA, C, O, OXT, CB, SG, HB2, HB3, HG                                                                         |
| CYS     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, SG, HB2, HB3, HG                                                                   |
| CYX     | n        | N, H1, H2, H3, CA, HA, C, O, CB, SG, HB2, HB3                                                                         |
| CYX     | n-       | N, H1, H2, CA, HA, C, O, CB, SG, HB2, HB3                                                                             |
| CYX     | m        | N, H, CA, HA, C, O, CB, SG, HB2, HB3                                                                                  |
| CYX     | c        | N, H, CA, HA, C, O, OXT, CB, SG, HB2, HB3                                                                             |
| CYX     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, SG, HB2, HB3                                                                       |
| GLH     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CG, OE1, OE2, HB2, HB3, HE2, HG2, HG3                                            |
| GLH     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CG, OE1, OE2, HB2, HB3, HE2, HG2, HG3                                                |
| GLH     | m        | N, H, CA, HA, C, O, CB, CD, CG, OE1, OE2, HB2, HB3, HE2, HG2, HG3                                                     |
| GLH     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CG, OE1, OE2, HB2, HB3, HE2, HG2, HG3                                                |
| GLH     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CG, OE1, OE2, HB2, HB3, HE2, HG2, HG3                                          |
| GLN     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CG, NE2, OE1, HB2, HB3, HE21, HE22, HG2, HG3                                     |
| GLN     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CG, NE2, OE1, HB2, HB3, HE21, HE22, HG2, HG3                                         |
| GLN     | m        | N, H, CA, HA, C, O, CB, CD, CG, NE2, OE1, HB2, HB3, HE21, HE22, HG2, HG3                                              |
| GLN     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CG, NE2, OE1, HB2, HB3, HE21, HE22, HG2, HG3                                         |
| GLN     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CG, NE2, OE1, HB2, HB3, HE21, HE22, HG2, HG3                                   |
| GLU     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CG, OE1, OE2, HB2, HB3, HG2, HG3                                                 |
| GLU     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CG, OE1, OE2, HB2, HB3, HG2, HG3                                                     |
| GLU     | m        | N, H, CA, HA, C, O, CB, CD, CG, OE1, OE2, HB2, HB3, HG2, HG3                                                          |
| GLU     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CG, OE1, OE2, HB2, HB3, HG2, HG3                                                     |
| GLU     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CG, OE1, OE2, HB2, HB3, HG2, HG3                                               |
| GLY     | n        | N, H1, H2, H3, CA, HA2, HA3, C, O                                                                                     |
| GLY     | n-       | N, H1, H2, CA, HA2, HA3, C, O                                                                                         |
| GLY     | m        | N, H, CA, HA2, HA3, C, O                                                                                              |
| GLY     | c        | N, H, CA, HA2, HA3, C, O, OXT                                                                                         |
| GLY     | c+       | N, H, CA, HA2, HA3, C, O, OXT, HOXT                                                                                   |
| HID     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1                                      |
| HID     | n-       | N, H1, H2, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1                                          |
| HID     | m        | N, H, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1                                               |
| HID     | c        | N, H, CA, HA, C, O, OXT, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1                                          |
| HID     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1                                    |
| HIE     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD2, HE1, HE2                                      |
| HIE     | n-       | N, H1, H2, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD2, HE1, HE2                                          |
| HIE     | m        | N, H, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD2, HE1, HE2                                               |
| HIE     | c        | N, H, CA, HA, C, O, OXT, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD2, HE1, HE2                                          |
| HIE     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD2, HE1, HE2                                    |
| HIP     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1, HE2                                 |
| HIP     | n-       | N, H1, H2, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1, HE2                                     |
| HIP     | m        | N, H, CA, HA, C, O, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1, HE2                                          |
| HIP     | c        | N, H, CA, HA, C, O, OXT, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1, HE2                                     |
| HIP     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD2, CE1, CG, ND1, NE2, HB2, HB3, HD1, HD2, HE1, HE2                               |
| ILE     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD1, CG1, CG2, HB, HD11, HD12, HD13, HG12, HG13, HG21, HG22, HG23                    |
| ILE     | n-       | N, H1, H2, CA, HA, C, O, CB, CD1, CG1, CG2, HB, HD11, HD12, HD13, HG12, HG13, HG21, HG22, HG23                        |
| ILE     | m        | N, H, CA, HA, C, O, CB, CD1, CG1, CG2, HB, HD11, HD12, HD13, HG12, HG13, HG21, HG22, HG23                             |
| ILE     | c        | N, H, CA, HA, C, O, OXT, CB, CD1, CG1, CG2, HB, HD11, HD12, HD13, HG12, HG13, HG21, HG22, HG23                        |
| ILE     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD1, CG1, CG2, HB, HD11, HD12, HD13, HG12, HG13, HG21, HG22, HG23                  |
| LEU     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD1, CD2, CG, HB2, HB3, HD11, HD12, HD13, HD21, HD22, HD23, HG                       |
| LEU     | n-       | N, H1, H2, CA, HA, C, O, CB, CD1, CD2, CG, HB2, HB3, HD11, HD12, HD13, HD21, HD22, HD23, HG                           |
| LEU     | m        | N, H, CA, HA, C, O, CB, CD1, CD2, CG, HB2, HB3, HD11, HD12, HD13, HD21, HD22, HD23, HG                                |
| LEU     | c        | N, H, CA, HA, C, O, OXT, CB, CD1, CD2, CG, HB2, HB3, HD11, HD12, HD13, HD21, HD22, HD23, HG                           |
| LEU     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD1, CD2, CG, HB2, HB3, HD11, HD12, HD13, HD21, HD22, HD23, HG                     |
| LYN     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2                     |
| LYN     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2                         |
| LYN     | m        | N, H, CA, HA, C, O, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2                              |
| LYN     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2                         |
| LYN     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2                   |
| LYS     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2, HZ3                |
| LYS     | n-       | N, H1, H2, CA, HA, C, O, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2, HZ3                    |
| LYS     | m        | N, H, CA, HA, C, O, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2, HZ3                         |
| LYS     | c        | N, H, CA, HA, C, O, OXT, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2, HZ3                    |
| LYS     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD, CE, CG, NZ, HB2, HB3, HD2, HD3, HE2, HE3, HG2, HG3, HZ1, HZ2, HZ3              |
| MET     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CE, CG, SD, HB2, HB3, HE1, HE2, HE3, HG2, HG3                                        |
| MET     | n-       | N, H1, H2, CA, HA, C, O, CB, CE, CG, SD, HB2, HB3, HE1, HE2, HE3, HG2, HG3                                            |
| MET     | m        | N, H, CA, HA, C, O, CB, CE, CG, SD, HB2, HB3, HE1, HE2, HE3, HG2, HG3                                                 |
| MET     | c        | N, H, CA, HA, C, O, OXT, CB, CE, CG, SD, HB2, HB3, HE1, HE2, HE3, HG2, HG3                                            |
| MET     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CE, CG, SD, HB2, HB3, HE1, HE2, HE3, HG2, HG3                                      |
| PHE     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, HB2, HB3, HD1, HD2, HE1, HE2, HZ                         |
| PHE     | n-       | N, H1, H2, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, HB2, HB3, HD1, HD2, HE1, HE2, HZ                             |
| PHE     | m        | N, H, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, HB2, HB3, HD1, HD2, HE1, HE2, HZ                                  |
| PHE     | c        | N, H, CA, HA, C, O, OXT, CB, CD1, CD2, CE1, CE2, CG, CZ, HB2, HB3, HD1, HD2, HE1, HE2, HZ                             |
| PHE     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD1, CD2, CE1, CE2, CG, CZ, HB2, HB3, HD1, HD2, HE1, HE2, HZ                       |
| PRO     | n        | N, H2, H3, CA, HA, C, O, CB, CD, CG, HB2, HB3, HD2, HD3, HG2, HG3                                                     |
| PRO     | n-       | N, H2, CA, HA, C, O, CB, CD, CG, HB2, HB3, HD2, HD3, HG2, HG3                                                         |
| PRO     | m        | N, CA, HA, C, O, CB, CD, CG, HB2, HB3, HD2, HD3, HG2, HG3                                                             |
| PRO     | c        | N, CA, HA, C, O, OXT, CB, CD, CG, HB2, HB3, HD2, HD3, HG2, HG3                                                        |
| PRO     | c+       | N, CA, HA, C, O, OXT, HOXT, CB, CD, CG, HB2, HB3, HD2, HD3, HG2, HG3                                                  |
| SER     | n        | N, H1, H2, H3, CA, HA, C, O, CB, OG, HB2, HB3, HG                                                                     |
| SER     | n-       | N, H1, H2, CA, HA, C, O, CB, OG, HB2, HB3, HG                                                                         |
| SER     | m        | N, H, CA, HA, C, O, CB, OG, HB2, HB3, HG                                                                              |
| SER     | c        | N, H, CA, HA, C, O, OXT, CB, OG, HB2, HB3, HG                                                                         |
| SER     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, OG, HB2, HB3, HG                                                                   |
| THR     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CG2, OG1, HB, HG1, HG21, HG22, HG23                                                  |
| THR     | n-       | N, H1, H2, CA, HA, C, O, CB, CG2, OG1, HB, HG1, HG21, HG22, HG23                                                      |
| THR     | m        | N, H, CA, HA, C, O, CB, CG2, OG1, HB, HG1, HG21, HG22, HG23                                                           |
| THR     | c        | N, H, CA, HA, C, O, OXT, CB, CG2, OG1, HB, HG1, HG21, HG22, HG23                                                      |
| THR     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CG2, OG1, HB, HG1, HG21, HG22, HG23                                                |
| TRP     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD1, CD2, CE2, CE3, CG, CH2, CZ2, CZ3, NE1, HB2, HB3, HD1, HE1, HE3, HH2, HZ2, HZ3   |
| TRP     | n-       | N, H1, H2, CA, HA, C, O, CB, CD1, CD2, CE2, CE3, CG, CH2, CZ2, CZ3, NE1, HB2, HB3, HD1, HE1, HE3, HH2, HZ2, HZ3       |
| TRP     | m        | N, H, CA, HA, C, O, CB, CD1, CD2, CE2, CE3, CG, CH2, CZ2, CZ3, NE1, HB2, HB3, HD1, HE1, HE3, HH2, HZ2, HZ3            |
| TRP     | c        | N, H, CA, HA, C, O, OXT, CB, CD1, CD2, CE2, CE3, CG, CH2, CZ2, CZ3, NE1, HB2, HB3, HD1, HE1, HE3, HH2, HZ2, HZ3       |
| TRP     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD1, CD2, CE2, CE3, CG, CH2, CZ2, CZ3, NE1, HB2, HB3, HD1, HE1, HE3, HH2, HZ2, HZ3 |
| TYM     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2                         |
| TYM     | n-       | N, H1, H2, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2                             |
| TYM     | m        | N, H, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2                                  |
| TYM     | c        | N, H, CA, HA, C, O, OXT, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2                             |
| TYM     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2                       |
| TYR     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2, HH                     |
| TYR     | n-       | N, H1, H2, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2, HH                         |
| TYR     | m        | N, H, CA, HA, C, O, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2, HH                              |
| TYR     | c        | N, H, CA, HA, C, O, OXT, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2, HH                         |
| TYR     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CD1, CD2, CE1, CE2, CG, CZ, OH, HB2, HB3, HD1, HD2, HE1, HE2, HH                   |
| VAL     | n        | N, H1, H2, H3, CA, HA, C, O, CB, CG1, CG2, HB, HG11, HG12, HG13, HG21, HG22, HG23                                     |
| VAL     | n-       | N, H1, H2, CA, HA, C, O, CB, CG1, CG2, HB, HG11, HG12, HG13, HG21, HG22, HG23                                         |
| VAL     | m        | N, H, CA, HA, C, O, CB, CG1, CG2, HB, HG11, HG12, HG13, HG21, HG22, HG23                                              |
| VAL     | c        | N, H, CA, HA, C, O, OXT, CB, CG1, CG2, HB, HG11, HG12, HG13, HG21, HG22, HG23                                         |
| VAL     | c+       | N, H, CA, HA, C, O, OXT, HOXT, CB, CG1, CG2, HB, HG11, HG12, HG13, HG21, HG22, HG23                                   |

---

## Nucleic Acid Residues

### DNA Residues

| Residue | Description    | Net Charge |
| ------- | -------------- | ---------- |
| DA      | Deoxyadenosine | −1         |
| DC      | Deoxycytidine  | −1         |
| DG      | Deoxyguanosine | −1         |
| DT      | Deoxythymidine | −1         |
| DI      | Deoxyinosine   | −1         |

### RNA Residues

| Residue | Description | Net Charge |
| ------- | ----------- | ---------- |
| A       | Adenosine   | −1         |
| C       | Cytidine    | −1         |
| G       | Guanosine   | −1         |
| U       | Uridine     | −1         |
| I       | Inosine     | −1         |

### Nucleic Acid Atom Names by Residue and Position

> **Position Key**: `5` = 5'-terminal, `m` = Middle, `3` = 3'-terminal

| Residue | Position | Atoms                                                                                                                                                             |
| ------- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A       | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N6, N7, N9, O2', O3', O4', O5', H1', H2, H2', H3', H4', H5', H5'', H61, H62, H8, HO2', HO5'                  |
| A       | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N6, N7, N9, O2', O3', O4', O5', OP1, OP2, P, H1', H2, H2', H3', H4', H5', H5'', H61, H62, H8, HO2'           |
| A       | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N6, N7, N9, O2', O3', O4', O5', OP1, OP2, P, H1', H2, H2', H3', H4', H5', H5'', H61, H62, H8, HO2', HO3'     |
| C       | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, N4, O2, O2', O3', O4', O5', H1', H2', H3', H4', H41, H42, H5, H5', H5'', H6, HO2', HO5'                          |
| C       | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, N4, O2, O2', O3', O4', O5', OP1, OP2, P, H1', H2', H3', H4', H41, H42, H5, H5', H5'', H6, HO2'                   |
| C       | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, N4, O2, O2', O3', O4', O5', OP1, OP2, P, H1', H2', H3', H4', H41, H42, H5, H5', H5'', H6, HO2', HO3'             |
| DA      | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N6, N7, N9, O3', O4', O5', H1', H2, H2', H2'', H3', H4', H5', H5'', H61, H62, H8, HO5'                       |
| DA      | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N6, N7, N9, O3', O4', O5', OP1, OP2, P, H1', H2, H2', H2'', H3', H4', H5', H5'', H61, H62, H8                |
| DA      | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N6, N7, N9, O3', O4', O5', OP1, OP2, P, H1', H2, H2', H2'', H3', H4', H5', H5'', H61, H62, H8, HO3'          |
| DC      | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, N4, O2, O3', O4', O5', H1', H2', H2'', H3', H4', H41, H42, H5, H5', H5'', H6, HO5'                               |
| DC      | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, N4, O2, O3', O4', O5', OP1, OP2, P, H1', H2', H2'', H3', H4', H41, H42, H5, H5', H5'', H6                        |
| DC      | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, N4, O2, O3', O4', O5', OP1, OP2, P, H1', H2', H2'', H3', H4', H41, H42, H5, H5', H5'', H6, HO3'                  |
| DG      | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N2, N3, N7, N9, O3', O4', O5', O6, H1, H1', H2', H2'', H21, H22, H3', H4', H5', H5'', H8, HO5'                   |
| DG      | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N2, N3, N7, N9, O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2', H2'', H21, H22, H3', H4', H5', H5'', H8            |
| DG      | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N2, N3, N7, N9, O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2', H2'', H21, H22, H3', H4', H5', H5'', H8, HO3'      |
| DI      | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N7, N9, O3', O4', O5', O6, H1, H1', H2, H2', H2'', H3', H4', H5', H5'', H8, HO5'                             |
| DI      | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N7, N9, O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2, H2', H2'', H3', H4', H5', H5'', H8                      |
| DI      | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N7, N9, O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2, H2', H2'', H3', H4', H5', H5'', H8, HO3'                |
| DT      | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C7, N1, N3, O2, O3', O4, O4', O5', H1', H2', H2'', H3, H3', H4', H5', H5'', H6, H71, H72, H73, HO5'                      |
| DT      | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C7, N1, N3, O2, O3', O4, O4', O5', OP1, OP2, P, H1', H2', H2'', H3, H3', H4', H5', H5'', H6, H71, H72, H73               |
| DT      | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C7, N1, N3, O2, O3', O4, O4', O5', OP1, OP2, P, H1', H2', H2'', H3, H3', H4', H5', H5'', H6, H71, H72, H73, HO3'         |
| G       | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N2, N3, N7, N9, O2', O3', O4', O5', O6, H1, H1', H2', H21, H22, H3', H4', H5', H5'', H8, HO2', HO5'              |
| G       | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N2, N3, N7, N9, O2', O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2', H21, H22, H3', H4', H5', H5'', H8, HO2'       |
| G       | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N2, N3, N7, N9, O2', O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2', H21, H22, H3', H4', H5', H5'', H8, HO2', HO3' |
| I       | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N7, N9, O2', O3', O4', O5', O6, H1, H1', H2, H2', H3', H4', H5', H5'', H8, HO2', HO5'                        |
| I       | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N7, N9, O2', O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2, H2', H3', H4', H5', H5'', H8, HO2'                 |
| I       | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, C8, N1, N3, N7, N9, O2', O3', O4', O5', O6, OP1, OP2, P, H1, H1', H2, H2', H3', H4', H5', H5'', H8, HO2', HO3'           |
| U       | 5        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, O2, O2', O3', O4, O4', O5', H1', H2', H3, H3', H4', H5, H5', H5'', H6, HO2', HO5'                                |
| U       | m        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, O2, O2', O3', O4, O4', O5', OP1, OP2, P, H1', H2', H3, H3', H4', H5, H5', H5'', H6, HO2'                         |
| U       | 3        | C1', C2, C2', C3', C4, C4', C5, C5', C6, N1, N3, O2, O2', O3', O4, O4', O5', OP1, OP2, P, H1', H2', H3, H3', H4', H5, H5', H5'', H6, HO2', HO3'                   |

---

## Water

Water uses the residue name `HOH` with atoms `O`, `H1`, and `H2`. Both hydrogen atoms have identical charges.

| Model    | O Charge | H Charge |
| -------- | -------- | -------- |
| TIP3P    | −0.834   | +0.417   |
| TIP3P-FB | −0.8484  | +0.4242  |
| SPC      | −0.82    | +0.41    |
| SPC/E    | −0.8476  | +0.4238  |
| OPC3     | −0.8952  | +0.4476  |

---

## Ions

All ions use formal (integer) charges.

### Cations

| Charge | Residue Names                                                                                                                |
| ------ | ---------------------------------------------------------------------------------------------------------------------------- |
| +1     | AG, AU, CS, CU1, K, LI, NA, RB, TL                                                                                           |
| +2     | BA, CA, CD, CO, CU, EU, FE2, HG, MG, MN, NI, PB, PD, PT, SR, ZN                                                              |
| +3     | 3CO, 3NI, AL, AU3, CE, CR, DY, ER3, EU3, FE, GA, GD3, HO3, IN, IR3, LA, LU, MN3, ND, OS, PR, RH3, RU, SB, SM, TB, V, YB, YT3 |
| +4     | 4MO, IR, OS4, PT4, TH, ZR                                                                                                    |
| +6     | 6MO, W                                                                                                                       |

### Anions

| Charge | Residue Names  |
| ------ | -------------- |
| −1     | BR, CL, F, IOD |
