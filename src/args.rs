use crate::tournament::format;
use std::path::PathBuf;

#[derive(clap::Parser, Clone, Debug, PartialEq, Eq)]
pub struct Args {
    /// .csv file to read data from, using '<player/team>,<class>' syntax, where <class> is optional
    pub file: PathBuf,
    #[arg(short, long, value_enum, default_value_t = format::Supported::DoubleElemination)]
    pub format: format::Supported,
}
