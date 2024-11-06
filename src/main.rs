use args::Args;
use clap::Parser;
use tournament::{backend, format, Tournament};

/// argument parsing
mod args;
/// the tournament itself: logic/backend
mod tournament;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    // let format: Box<dyn format::Format<backend::Cli>> = match args.format {
    //     format::Supported::SingleElemination => Box::new(format::SingleElemination::default()),
    //     format::Supported::DoubleElemination => Box::new(format::DoubleElemination::default()),
    //     format::Supported::RoundRobin => todo!(),
    //     format::Supported::SwissSystem => todo!(),
    // };

    // new tournament, communicate with the user via the cli
    match args.format {
        format::Supported::SingleElemination => {
            Tournament::new(backend::Cli, format::SingleElemination::default())
                .players_from_path(args.file)?
                .run();
        }
        format::Supported::DoubleElemination => {
            Tournament::new(backend::Cli, format::DoubleElemination::default())
                .players_from_path(args.file)?
                .run();
        }
        format::Supported::RoundRobin => {
            todo!();
        }
        format::Supported::SwissSystem => {
            todo!();
        }
    }
    // let mut tournament = Tournament::new(backend::Cli, format).players_from_path(args.file)?;

    // players.save();

    Ok(())

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
