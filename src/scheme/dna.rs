//! DNA charge schemes.

/// DNA charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DnaScheme {
    /// AMBER (AMBER OL15, OL21, OL24, bsc1).
    #[default]
    Amber,
    /// CHARMM (CHARMM C27, C36).
    Charmm,
}

impl DnaScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Amber => "amber",
            Self::Charmm => "charmm",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[Self::Amber, Self::Charmm]
    }

    /// Returns the partial charge for an atom in a DNA residue.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the residue in the chain.
    /// * `residue` - Residue name (e.g., "DA", "DG").
    /// * `atom` - Atom name (e.g., "C1'", "H2''").
    ///
    /// # Returns
    ///
    /// `Option<f32>` - Partial charge if found, otherwise `None`.
    pub fn charge(self, pos: crate::Position, residue: &str, atom: &str) -> Option<f32> {
        crate::generated::get_nucleic_charge(self.key(), pos.key(), residue, atom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(DnaScheme::default(), DnaScheme::Amber);
    }

    #[test]
    fn count() {
        assert_eq!(DnaScheme::all().len(), 2);
    }

    #[test]
    fn key_format() {
        for s in DnaScheme::all() {
            let k = s.key();
            assert!(
                k.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
            );
        }
    }
}
