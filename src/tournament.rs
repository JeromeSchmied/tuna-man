use std::path::Path;

use players::Players;
use structs::Match;

mod players;
mod structs;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Tournament {
    pub(crate) winner_branch: Vec<Match>,
    pub(crate) loser_branch: Vec<Match>,
    pub(crate) knocked: Players,
}

impl Tournament {
    // pub fn with_tables(self, tables: &[Table]) -> Self {
    //     Self {
    //         tables: tables.into(),
    //         ..self
    //     }
    // }
    pub(crate) fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let players = Players::load(path)?;
        Ok(Self::from(players))
    }
    pub(crate) fn is_end(&self) -> bool {
        self.winner_branch.is_empty() && self.loser_branch.is_empty()
    }
    pub(crate) fn play_next_round(&mut self) {
        let mut new_win = Players::default();
        let mut new_lose = Players::default();
        let mut knocked = std::mem::take(&mut self.knocked);
        // get outcomes
        while let Some(w_match) = self.winner_branch.pop() {
            if w_match.guest.is_unset() {
                new_win.0.push(w_match.homie);
                break;
            }
            println!("\nwinner match: {w_match}");
            let (winner, loser) = w_match.play();
            new_win.0.push(winner);
            new_lose.0.push(loser);
        }
        println!("\n-----------------------------");
        while let Some(l_match) = self.loser_branch.pop() {
            if l_match.guest.is_unset() {
                new_lose.0.push(l_match.homie);
                break;
            }
            println!("\nloser match: {l_match}");
            let (winner, loser) = l_match.play();
            new_lose.0.push(winner);
            println!("bye-bye {loser}");
            knocked.0.push(loser);
        }

        if new_win.0.len() == 1 {
            println!(
                "probably should end soon, only winner branch remainder: {}",
                new_win.0[0]
            );
        } else if new_win.0.len() % 2 == 1 {
            new_win.shuffle_as_pairs(); // shuffle
            let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_match = Match {
                homie,
                guest,
                outcome: None,
            }; // create a match
            println!("\nspecial winner match: {w_match}");
            let (winner, loser) = w_match.play(); // play it
            new_win.0.push(winner); // winner stays
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }

        if new_lose.0.len() == 1 {
            let homie = new_win.0.pop().unwrap();
            let guest = new_lose.0.pop().unwrap();
            let finals = Match {
                homie,
                guest,
                outcome: None,
            };
            println!("FINAL GAME: {finals}");
            let (winner, second) = finals.play();
            knocked.0.push(second);
            knocked.0.push(winner);
        } else if new_lose.0.len() % 2 == 1 {
            new_lose.shuffle_as_pairs(); // shuffle
            let (homie, guest) = (new_lose.0.swap_remove(0), new_lose.0.swap_remove(0)); // remove first two
            let l_match = Match {
                homie,
                guest,
                outcome: None,
            }; // create a match
            println!("\nspecial loser match: {l_match}");
            let (winner, loser) = l_match.play(); // play it
            new_lose.0.push(winner); // winner stays
            println!("bye-bye {loser}"); // loser get's eleminated
            knocked.0.push(loser);
        }
        // dbg!(&new_win);
        // dbg!(&new_lose);
        *self = Self {
            winner_branch: new_win.into(),
            loser_branch: new_lose.into(),
            knocked,
        };
    }
    // pub fn execute(
    //     &mut self,
    //     term: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
    // ) -> std::io::Result<()> {
    //     todo!("app logic");
    //     loop {
    //         term.draw(|f| self.ui(f))?;

    //         if let Event::Key(key) = event::read()? {
    //             if key.kind != KeyEventKind::Press {
    //                 continue;
    //             }
    //             match key.code {
    //                 KeyCode::Char('q') | KeyCode::Esc => break,
    //                 KeyCode::Char('j') | KeyCode::Down => todo!(),
    //                 KeyCode::Char('k') | KeyCode::Up => todo!(),
    //                 KeyCode::Char('r') => todo!(),
    //                 KeyCode::Char('n') | KeyCode::Char('l') | KeyCode::Right => todo!(),
    //                 KeyCode::Char('p') | KeyCode::Char('h') | KeyCode::Left => todo!(),
    //                 KeyCode::Char('R') | KeyCode::Backspace => todo!(),
    //                 _ => {}
    //             }
    //         } else {
    //             // resize and restart
    //         }
    //     }
    //     Ok(())
    // }
}

impl From<Players> for Tournament {
    fn from(players: Players) -> Self {
        assert!(
            players.0.len() >= 3,
            "you need at least 3 participants to play a tournament"
        );

        let mut new_win = players;
        let mut new_lose = Players::default();
        if new_win.0.len() % 2 == 1 {
            new_win.shuffle_as_pairs(); // shuffle
            let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_match = Match {
                homie,
                guest,
                outcome: None,
            }; // create a match
            println!("\nspecial winner match: {w_match}");
            let (winner, loser) = w_match.play(); // play it
            new_win.0.push(winner); // winner stays
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }
        Self {
            winner_branch: new_win.into(),
            loser_branch: new_lose.into(),
            knocked: Players::default(),
        }
    }
}
