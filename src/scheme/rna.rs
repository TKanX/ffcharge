//! RNA charge schemes.

/// RNA charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RnaScheme {
    /// AMBER (AMBER OL3).
    #[default]
    Amber,
    /// CHARMM (CHARMM C27, C36).
    Charmm,
}

impl RnaScheme {
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

    /// Returns the partial charge for an atom in an RNA residue.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the residue in the chain.
    /// * `residue` - Residue name (e.g., "A", "G").
    /// * `atom` - Atom name (e.g., "C1'", "H2'").
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
        assert_eq!(RnaScheme::default(), RnaScheme::Amber);
    }

    #[test]
    fn count() {
        assert_eq!(RnaScheme::all().len(), 2);
    }

    #[test]
    fn key_format() {
        for s in RnaScheme::all() {
            let k = s.key();
            assert!(
                k.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
            );
        }
    }
}
