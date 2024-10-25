use std::io::Write;

use super::{players::Players, structs::Duel, Tournament};

pub trait Backend {
    fn shuffle(players: &mut Players);
    fn get_outcome(duel: &Duel) -> Result<Duel, ()>;

    /// double-knockout
    fn play_round(tournament: &mut Tournament<impl Backend>) {
        let mut new_win = Players::default();
        let mut new_lose = Players::default();
        let knocked = &mut tournament.knocked;
        // get outcomes
        while let Some(w_duel) = tournament.winner_branch.pop() {
            if w_duel.guest.is_unset() {
                new_win.0.push(w_duel.homie);
                break;
            }
            eprintln!("\nwinner duel: {w_duel}");
            let (winner, loser) = w_duel.play(Self::get_outcome);
            new_win.0.push(winner);
            new_lose.0.push(loser);
        }
        eprintln!("\n-----------------------------");
        while let Some(l_duel) = tournament.loser_branch.pop() {
            if l_duel.guest.is_unset() {
                new_lose.0.push(l_duel.homie);
                break;
            }
            eprintln!("\nloser duel: {l_duel}");
            let (winner, loser) = l_duel.play(Self::get_outcome);
            new_lose.0.push(winner);
            eprintln!("bye-bye {loser}");
            knocked.0.push(loser);
        }

        if new_win.0.len() == 1 {
            eprintln!(
                "probably should end soon, only winner branch remainder: {}",
                new_win.0[0]
            );
        } else if new_win.0.len() % 2 == 1 {
            new_win.shuffle_as_pairs(Self::shuffle); // make a suitable duel
            let (homie, guest) = (new_win.0.remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_duel = Duel {
                homie,
                guest,
                outcome: None,
            }; // create a duel
            eprintln!("\nspecial winner duel: {w_duel}");
            let (winner, loser) = w_duel.play(Self::get_outcome); // play it
            new_win.0.push(winner); // winner stays
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }

        if new_lose.0.len() == 1 {
            let homie = new_win.0.pop().unwrap();
            let guest = new_lose.0.pop().unwrap();
            let finals = Duel {
                homie,
                guest,
                outcome: None,
            };
            eprintln!("FINAL GAME: {finals}");
            let (winner, second) = finals.play(Self::get_outcome);
            knocked.0.push(second);
            knocked.0.push(winner);
        } else if new_lose.0.len() % 2 == 1 {
            new_lose.shuffle_as_pairs(Self::shuffle); // make a suitable duel
            let (homie, guest) = (new_lose.0.remove(0), new_lose.0.swap_remove(0)); // remove first two
            let l_duel = Duel {
                homie,
                guest,
                outcome: None,
            }; // create a duel
            eprintln!("\nspecial loser duel: {l_duel}");
            let (winner, loser) = l_duel.play(Self::get_outcome); // play it
            new_lose.0.push(winner); // winner stays
            eprintln!("bye-bye {loser}"); // loser get's eleminated
            knocked.0.push(loser);
        }
        // dbg!(&new_win);
        // dbg!(&new_lose);
        tournament.winner_branch = new_win.into_vec_duel(Self::shuffle);
        tournament.loser_branch = new_lose.into_vec_duel(Self::shuffle);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test;
impl Backend for Test {
    fn shuffle(_players: &mut Players) {}

    fn get_outcome(duel: &Duel) -> Result<Duel, ()> {
        Ok(duel.clone().with_outcome(Some(true)))
    }
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cli;
impl Backend for Cli {
    fn shuffle(players: &mut Players) {
        fastrand::shuffle(&mut players.0);
    }

    fn get_outcome(duel: &Duel) -> Result<Duel, ()> {
        print!("winner: ");
        std::io::stdout().flush().map_err(|_| ())?;
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).map_err(|_| ())?;
        let outcome = match buf.trim() {
            "<" | "homie" => Some(true),
            ">" | "guest" => Some(false),
            name => {
                let name = name.to_lowercase();
                if duel.homie.name.to_lowercase().contains(&name) {
                    Some(true)
                } else if duel.guest.name.to_lowercase().contains(&name) {
                    Some(false)
                } else {
                    // dbg!(&name);
                    if matches!(name.as_str(), "q" | "quit" | "exit") {
                        std::process::exit(0);
                    }
                    return Err(());
                }
            }
        };
        // println!("{duel}");
        Ok(duel.clone().with_outcome(outcome))
    }
}
