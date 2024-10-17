use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// .csv file to read data from, using 'player, grade' syntax
    #[arg(short, long)]
    pub file: PathBuf,
}
