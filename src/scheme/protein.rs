//! Protein charge schemes.

/// Protein charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ProteinScheme {
    /// AMBER ff14SB.
    #[default]
    AmberFF14SB,
    /// AMBER ff19SB.
    AmberFF19SB,
    /// AMBER ff99SB.
    AmberFF99SB,
    /// AMBER ff99SB-ILDN.
    AmberFF99SBILDN,
    /// AMBER ff03.
    AmberFF03,
    /// CHARMM36.
    CharmmC36,
    /// CHARMM36m.
    CharmmC36m,
    /// CHARMM27.
    CharmmC27,
    /// CHARMM22.
    CharmmC22,
    /// CHARMM22/CMAP.
    CharmmC22CMAP,
}

impl ProteinScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::AmberFF14SB => "amber-ff14sb",
            Self::AmberFF19SB => "amber-ff19sb",
            Self::AmberFF99SB => "amber-ff99sb",
            Self::AmberFF99SBILDN => "amber-ff99sb-ildn",
            Self::AmberFF03 => "amber-ff03",
            Self::CharmmC36 => "charmm-c36",
            Self::CharmmC36m => "charmm-c36m",
            Self::CharmmC27 => "charmm-c27",
            Self::CharmmC22 => "charmm-c22",
            Self::CharmmC22CMAP => "charmm-c22-cmap",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[
            Self::AmberFF14SB,
            Self::AmberFF19SB,
            Self::AmberFF99SB,
            Self::AmberFF99SBILDN,
            Self::AmberFF03,
            Self::CharmmC36,
            Self::CharmmC36m,
            Self::CharmmC27,
            Self::CharmmC22,
            Self::CharmmC22CMAP,
        ]
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
