//! DNA charge schemes.

/// DNA charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DnaScheme {
    /// AMBER OL15.
    #[default]
    AmberOL15,
    /// AMBER bsc1.
    AmberBSC1,
    /// AMBER OL21.
    AmberOL21,
    /// CHARMM36.
    CharmmC36,
    /// CHARMM27.
    CharmmC27,
}

impl DnaScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::AmberOL15 => "amber-ol15",
            Self::AmberBSC1 => "amber-bsc1",
            Self::AmberOL21 => "amber-ol21",
            Self::CharmmC36 => "charmm-c36",
            Self::CharmmC27 => "charmm-c27",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[
            Self::AmberOL15,
            Self::AmberBSC1,
            Self::AmberOL21,
            Self::CharmmC36,
            Self::CharmmC27,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(DnaScheme::default(), DnaScheme::AmberOL15);
    }

    #[test]
    fn count() {
        assert_eq!(DnaScheme::all().len(), 5);
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
