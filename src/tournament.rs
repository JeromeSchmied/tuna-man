use std::path::Path;

use players::Players;
use structs::Duel;

mod players;
mod structs;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Tournament {
    pub(crate) winner_branch: Vec<Duel>,
    pub(crate) loser_branch: Vec<Duel>,
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
    pub(crate) fn play_cli(mut self) -> Self {
        let mut new_win = Players::default();
        let mut new_lose = Players::default();
        let mut knocked = self.knocked;
        // get outcomes
        while let Some(w_duel) = self.winner_branch.pop() {
            if w_duel.guest.is_unset() {
                new_win.0.push(w_duel.homie);
                break;
            }
            println!("\nwinner duel: {w_duel}");
            let (winner, loser) = w_duel.play();
            new_win.0.push(winner);
            new_lose.0.push(loser);
        }
        println!("\n-----------------------------");
        while let Some(l_duel) = self.loser_branch.pop() {
            if l_duel.guest.is_unset() {
                new_lose.0.push(l_duel.homie);
                break;
            }
            println!("\nloser duel: {l_duel}");
            let (winner, loser) = l_duel.play();
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
            new_win.shuffle_as_pairs(Players::shuffle); // shuffle
            let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_duel = Duel {
                homie,
                guest,
                outcome: None,
            }; // create a duel
            println!("\nspecial winner duel: {w_duel}");
            let (winner, loser) = w_duel.play(); // play it
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
            println!("FINAL GAME: {finals}");
            let (winner, second) = finals.play();
            knocked.0.push(second);
            knocked.0.push(winner);
        } else if new_lose.0.len() % 2 == 1 {
            new_lose.shuffle_as_pairs(Players::shuffle); // shuffle
            let (homie, guest) = (new_lose.0.swap_remove(0), new_lose.0.swap_remove(0)); // remove first two
            let l_duel = Duel {
                homie,
                guest,
                outcome: None,
            }; // create a duel
            println!("\nspecial loser duel: {l_duel}");
            let (winner, loser) = l_duel.play(); // play it
            new_lose.0.push(winner); // winner stays
            println!("bye-bye {loser}"); // loser get's eleminated
            knocked.0.push(loser);
        }
        // dbg!(&new_win);
        // dbg!(&new_lose);
        Self {
            winner_branch: new_win.into(),
            loser_branch: new_lose.into(),
            knocked,
        }
    }
    pub(crate) fn play_next_round(&mut self, play_round: impl FnOnce(Self) -> Self) {
        let temp_tournament = std::mem::take(self);
        *self = play_round(temp_tournament);
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
            new_win.shuffle_as_pairs(Players::shuffle); // shuffle
            let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_duel = Duel {
                homie,
                guest,
                outcome: None,
            }; // create a duel
            println!("\nspecial winner duel: {w_duel}");
            let (winner, loser) = w_duel.play(); // play it
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
