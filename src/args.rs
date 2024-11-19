use crate::tournament::format;
use std::path::PathBuf;

#[derive(clap::Parser, Clone, Debug, PartialEq, Eq)]
pub struct Args {
    /// Path to file with participants: '<player/team>,<class>' syntax, where <class> is optional
    pub file: PathBuf,
    /// Format in which the Tournament shall be carried out
    #[arg(short, long, value_enum, default_value_t = format::Supported::DoubleElemination)]
    pub format: format::Supported,
    /// When to smart-shuffle players
    /// NOTE: ignored if <format> is not elemination type
    #[arg(short, long, value_enum, default_value_t = Shuffle::Initially)]
    pub shuffle: Shuffle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum Shuffle {
    Always,
    Initially,
    Never,
}
impl Shuffle {
    pub fn always(self) -> bool {
        self == Self::Always
    }
    pub fn initially(self) -> bool {
        self == Self::Initially
    }
    pub fn never(self) -> bool {
        self == Self::Never
    }
}
