//! Residue position within a molecular chain.

/// Position of a residue in its chain.
///
/// Terminal residues have different charge distributions than internal ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Position {
    /// N-terminal (protein).
    NTerminal,
    /// C-terminal (protein).
    CTerminal,
    /// 5'-terminal (nucleic acid).
    FivePrime,
    /// 3'-terminal (nucleic acid).
    ThreePrime,
    /// Internal residue.
    #[default]
    Middle,
}

impl Position {
    /// Returns the key for this position.
    pub const fn key(self) -> &'static str {
        match self {
            Self::NTerminal => "n",
            Self::CTerminal => "c",
            Self::FivePrime => "5",
            Self::ThreePrime => "3",
            Self::Middle => "m",
        }
    }
}
