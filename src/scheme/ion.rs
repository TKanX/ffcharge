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

    /// Returns the partial charge for an ion residue.
    ///
    /// # Arguments
    ///
    /// * `residue` - Residue name (e.g., "NA", "CL").
    ///
    /// # Returns
    ///
    /// `Option<f32>` - Partial charge if found, otherwise `None`.
    pub fn charge(self, residue: &str) -> Option<f32> {
        crate::generated::get_ion_charge(self.key(), residue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(IonScheme::default(), IonScheme::Classic);
    }

    #[test]
    fn count() {
        assert_eq!(IonScheme::all().len(), 1);
    }

    #[test]
    fn key_format() {
        for s in IonScheme::all() {
            let k = s.key();
            assert!(
                k.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
            );
        }
    }
}
