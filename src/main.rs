use args::Args;
use clap::Parser;
use tournament::{backend, Tournament};

/// argument parsing
mod args;
/// the tournament itself: logic/backend
mod tournament;

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // new tournament, communicate with the user via the cli
    let mut tournament = Tournament::new(backend::Cli).players_from_path(args.file)?;

    // number of rounds
    let mut round = 0;

    // run till we've got all the results
    while !tournament.is_end() {
        // winner branch duels this round
        println!("\n\n\n\nRound {round}.\n--------\n\nWinner branch duels:\n");
        for w_duel in &tournament.winner_branch {
            println!("    {w_duel}");
        }
        // loser branch duels this round
        println!("\n-----------------------------\n\nLosing branch duels:\n");
        for l_duel in &tournament.loser_branch {
            println!("    {l_duel}");
        }
        println!("\n-----------------------------\n\n");

        tournament.play_next_round();

        round += 1;
    }

    // printing results
    println!("\nTournament ended in {round} rounds, Results:");
    println!("\n\nPODIUM\n------\n");
    println!("Winner: {}", tournament.knocked.0.pop().unwrap());
    println!("Second place: {}", tournament.knocked.0.pop().unwrap());
    println!("Third place: {}", tournament.knocked.0.pop().unwrap());
    println!("\nrunner-ups\n");
    for (place, player) in tournament.knocked.0.iter().rev().enumerate() {
        println!("{}. place: {player}", place + 4);
    }

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
