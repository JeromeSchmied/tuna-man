use std::path::PathBuf;

#[derive(clap::Parser, Clone, Debug, PartialEq, Eq)]
pub struct Args {
    /// .csv file to read data from, using '<player/team>,<class>' syntax, where <class> is optional
    pub file: PathBuf,
}
