//! Ion charge schemes.

/// Ion charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum IonScheme {
    /// Classic (formal charges).
    #[default]
    Classic,
}

impl IonScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Classic => "classic",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[Self::Classic]
    }
}
