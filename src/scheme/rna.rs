//! RNA charge schemes.

/// RNA charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum RnaScheme {
    /// AMBER OL3.
    #[default]
    AmberOL3,
    /// CHARMM36.
    CharmmC36,
    /// CHARMM27.
    CharmmC27,
}

impl RnaScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::AmberOL3 => "amber-ol3",
            Self::CharmmC36 => "charmm-c36",
            Self::CharmmC27 => "charmm-c27",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[Self::AmberOL3, Self::CharmmC36, Self::CharmmC27]
    }
}
