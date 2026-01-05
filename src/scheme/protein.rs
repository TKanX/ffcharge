//! Protein charge schemes.

/// Protein charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProteinScheme {
    /// AMBER ffSB (AMBER ff99SB, ff14SB, ff19SB).
    #[default]
    AmberFFSB,
    /// AMBER ff03.
    AmberFF03,
    /// CHARMM (CHARMM22, CHARMM27, CHARMM22/CMAP, CHARMM36, CHARMM36m).
    Charmm,
}

impl ProteinScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::AmberFFSB => "amber-ffsb",
            Self::AmberFF03 => "amber-ff03",
            Self::Charmm => "charmm",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[Self::AmberFFSB, Self::AmberFF03, Self::Charmm]
    }

    /// Returns the partial charge for an atom in a protein residue.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the residue in the chain.
    /// * `residue` - Residue name (e.g., "ALA", "ARG").
    /// * `atom` - Atom name (e.g., "CA", "HB1").
    ///
    /// # Returns
    ///
    /// `Option<f32>` - Partial charge if found, otherwise `None`.
    pub fn charge(self, pos: crate::Position, residue: &str, atom: &str) -> Option<f32> {
        crate::generated::get_protein_charge(self.key(), pos.key(), residue, atom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(ProteinScheme::default(), ProteinScheme::AmberFFSB);
    }

    #[test]
    fn count() {
        assert_eq!(ProteinScheme::all().len(), 3);
    }

    #[test]
    fn key_format() {
        for s in ProteinScheme::all() {
            let k = s.key();
            assert!(
                k.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
            );
        }
    }
}
