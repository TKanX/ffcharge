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
}
