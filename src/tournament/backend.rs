use super::{players::Players, structs::Duel, Tournament};
use std::io::Write;

/// Internal features necessary for retrieving information about a [`Tournament`]
pub trait Backend {
    /// how to shuffle the `players` before matching them into [`Duel`]s
    fn shuffle(players: &mut Players);
    /// how to retrieve outcome of a [`Duel`]
    fn get_outcome(duel: Duel) -> Result<Duel, ()>;

    /// play a round of a [`Tournament`]
    /// double-knockout for now
    // TODO: support more formats
    fn play_round(tournament: &mut Tournament<impl Backend>)
    where
        Self: std::marker::Sized,
    {
        // winner branch of the next round
        let mut new_win_b = Players::default();
        // loser branch of the next round
        let mut new_lose_b = Players::default();
        // knocked players of the next round
        let knocked = &mut tournament.knocked;

        // get outcomes for winner branch duels, move contestants to other branch if necessary
        while let Some(w_duel) = tournament.winner_branch.pop() {
            // duel isn't ready yet to be played, waiting for opponent
            if w_duel.guest.is_unset() {
                new_win_b.0.push(w_duel.homie); // should get into the next-round winner branch
                continue;
            }
            println!("\nwinner duel: {w_duel}");
            // play the duel, that leads us to having the result
            let (winner, loser) = w_duel.play(Self::get_outcome);
            new_win_b.0.push(winner); // winner get's to winner branch
            new_lose_b.0.push(loser); // loser get's to loser branch
        }
        println!("\n-----------------------------");

        // get outcomes for loser branch duels, move contestants to other branch if necessary
        while let Some(l_duel) = tournament.loser_branch.pop() {
            // duel isn't ready yet to be played, waiting for opponent
            if l_duel.guest.is_unset() {
                new_lose_b.0.push(l_duel.homie); // should get into the next-round loser branch
                continue;
            }
            println!("\nloser duel: {l_duel}");
            // play the duel, that leads us to having the result
            let (winner, loser) = l_duel.play(Self::get_outcome);
            new_lose_b.0.push(winner); // winner get's to loser branch
            println!("bye-bye {loser}");
            knocked.0.push(loser); // loser get's knocked out of the tournament
        }

        // handle special cases on winner branch
        if new_win_b.0.len() == 1 {
            println!(
                "soon final: only winner branch remainder: {}",
                new_win_b.0[0]
            );
        } else if new_win_b.0.len() % 2 == 1 {
            // not divisible by 2: we need a special pre-match: duel
            print!("\nspecial winner duel: ");
            let loser = Duel::handle_special::<Self>(&mut new_win_b);
            new_lose_b.0.push(loser); // loser get's pushed to loser branch
        }

        // handle special cases on loser branch
        if new_lose_b.0.len() == 1 {
            // final game: only player from winner and loser branch
            let homie = new_win_b.0.pop().unwrap();
            let guest = new_lose_b.0.pop().unwrap();
            let finals = Duel::new(homie, guest);
            println!("FINAL GAME: {finals}");
            let (winner, second) = finals.play(Self::get_outcome);
            // NOTE: everyone get's to the knocked players' list,
            // as it turns into the leaderboard if reversed
            knocked.0.push(second);
            knocked.0.push(winner);
        } else if new_lose_b.0.len() % 2 == 1 {
            // not divisible by 2: we need a special pre-match: duel
            println!("\nspecial loser duel: ");
            let loser = Duel::handle_special::<Self>(&mut new_lose_b);
            println!("bye-bye {loser}");
            knocked.0.push(loser); // loser get's eleminated: knocked out
        }
        // and we apply the changes by turning new branches into duels
        tournament.winner_branch = new_win_b.into_duels(Self::shuffle);
        tournament.loser_branch = new_lose_b.into_duels(Self::shuffle);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test;
impl Backend for Test {
    fn shuffle(_players: &mut Players) {}

    fn get_outcome(duel: Duel) -> Result<Duel, ()> {
        Ok(duel.clone().with_outcome(Some(true)))
    }
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cli;
impl Backend for Cli {
    /// randomly shuffle
    fn shuffle(players: &mut Players) {
        fastrand::shuffle(&mut players.0);
    }

    /// we read from `stdin`
    fn get_outcome(duel: Duel) -> Result<Duel, ()> {
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
