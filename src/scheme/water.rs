//! Water charge schemes.

/// Water charge scheme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WaterScheme {
    /// TIP3P.
    #[default]
    Tip3p,
    /// TIP3P-FB.
    Tip3pFb,
    /// SPC.
    Spc,
    /// SPC/E.
    SpcE,
    /// OPC3.
    Opc3,
}

impl WaterScheme {
    /// Returns the key for this scheme.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Tip3p => "tip3p",
            Self::Tip3pFb => "tip3p-fb",
            Self::Spc => "spc",
            Self::SpcE => "spc-e",
            Self::Opc3 => "opc3",
        }
    }

    /// Returns all available schemes.
    pub const fn all() -> &'static [Self] {
        &[
            Self::Tip3p,
            Self::Tip3pFb,
            Self::Spc,
            Self::SpcE,
            Self::Opc3,
        ]
    }

    /// Returns the partial charges for a water molecule.
    ///
    /// # Returns
    ///
    /// `Option<crate::WaterCharges>` - Partial charges if found, otherwise `None`.
    pub fn charges(self) -> Option<crate::WaterCharges> {
        crate::generated::get_water_charges(self.key())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(WaterScheme::default(), WaterScheme::Tip3p);
    }

    #[test]
    fn count() {
        assert_eq!(WaterScheme::all().len(), 5);
    }

    #[test]
    fn key_format() {
        for s in WaterScheme::all() {
            let k = s.key();
            assert!(
                k.chars()
                    .all(|c| c.is_ascii_lowercase() || c == '-' || c.is_ascii_digit())
            );
        }
    }
}
