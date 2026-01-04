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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(ProteinScheme::default(), ProteinScheme::AmberFF14SB);
    }

    #[test]
    fn count() {
        assert_eq!(ProteinScheme::all().len(), 10);
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
