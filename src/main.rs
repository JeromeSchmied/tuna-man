use args::Args;
use clap::Parser;
use tournament::{format, Tournament};

/// argument parsing
mod args;
/// the tournament itself: logic/backend
mod tournament;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    // let format = args.format.to_format();
    match args.format {
        format::Supported::SingleElemination => {
            Tournament::new(format::SingleElemination::default()).execute(args)
        }
        format::Supported::DoubleElemination => {
            Tournament::new(format::DoubleElemination::default()).execute(args)
        }
        format::Supported::RoundRobin => {
            Tournament::new(format::RoundRobin::default()).execute(args)
        }
        format::Supported::SwissSystem => {
            Tournament::new(format::SwissSystem::default()).execute(args)
        }
    }

    // TODO: ratatui ui
    // let mut terminal = ratatui::try_init()?;
    // let res = App::default().execute(&mut terminal);
    // ratatui::try_restore()?;
    // res
}
#[test]
fn does_it_contain() {
    let hay = "";
    let haystack = ["One Two", "Three Four", "Plum Pear"];
    assert!(haystack.iter().any(|s| s.contains(hay))); // NOTE: wow! it does.
}
