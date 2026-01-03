//! Residue position within a molecular chain.

/// Position of a residue in its chain.
///
/// Terminal residues have different charge distributions than internal ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Position {
    /// N-terminal (protein).
    NTerminal,
    /// N-terminal deprotonated (protein).
    NTerminalDeprotonated,
    /// C-terminal (protein).
    CTerminal,
    /// C-terminal protonated (protein).
    CTerminalProtonated,
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
            Self::NTerminalDeprotonated => "n-",
            Self::CTerminal => "c",
            Self::CTerminalProtonated => "c+",
            Self::FivePrime => "5",
            Self::ThreePrime => "3",
            Self::Middle => "m",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys() {
        assert_eq!(Position::NTerminal.key(), "n");
        assert_eq!(Position::NTerminalDeprotonated.key(), "n-");
        assert_eq!(Position::CTerminal.key(), "c");
        assert_eq!(Position::CTerminalProtonated.key(), "c+");
        assert_eq!(Position::FivePrime.key(), "5");
        assert_eq!(Position::ThreePrime.key(), "3");
        assert_eq!(Position::Middle.key(), "m");
    }

    #[test]
    fn default_is_middle() {
        assert_eq!(Position::default(), Position::Middle);
    }
}
