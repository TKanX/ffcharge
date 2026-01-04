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
