use std::process::exit;

use args::Args;
use clap::Parser;

use tournament::Tournament;
// use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

mod args;
mod tournament;
mod ui;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if let Some(f_path) = args.file.as_deref() {
        // let tables = vec![Table::default(); 4];
        let mut tournament = Tournament::from_path(f_path)?;
        let mut i = 0;
        while !tournament.is_end() {
            println!("\n\n\n\nRound {i}.\n--------\n\nWinner branch matches:\n");
            for w_match in &tournament.winner_branch {
                println!("    {w_match}");
            }
            println!("\n-----------------------------\n\nLosing branch matches:\n");
            for l_match in &tournament.loser_branch {
                println!("    {l_match}");
            }
            println!("\n-----------------------------\n\n");
            tournament.play_next_round();
            i += 1;
        }
        println!("\nTournament ended in {i} rounds, Results:");
        println!("\n\nPODIUM\n------\n");
        println!("Winner: {}", tournament.knocked.0.pop().unwrap());
        println!("Second place: {}", tournament.knocked.0.pop().unwrap());
        println!("Third place: {}", tournament.knocked.0.pop().unwrap());
        println!("\nrunner-ups\n");
        for (place, player) in tournament.knocked.0.iter().rev().enumerate() {
            println!("{}. place: {player}", place + 4);
        }
    } else {
        // TODO proper error handling
        // Handle the case where no file path is provided
        eprintln!("Error: No file path provided.");
        exit(1);
    }
    // players.save();

    Ok(())

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
// TODO: actual tests
