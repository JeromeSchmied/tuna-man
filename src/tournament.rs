use backend::Backend;
use players::Players;
use std::path::Path;
use structs::Duel;

pub mod backend;
mod players;
mod structs;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct Tournament<B: Backend> {
    pub(crate) winner_branch: Vec<Duel>,
    pub(crate) loser_branch: Vec<Duel>,
    pub(crate) knocked: Players,
    _backend: B,
}

impl<B: Backend> Tournament<B> {
    // pub fn with_tables(self, tables: &[Table]) -> Self {
    //     Self {
    //         tables: tables.into(),
    //         ..self
    //     }
    // }
    pub fn new(backend: B) -> Self {
        Self {
            winner_branch: vec![],
            loser_branch: vec![],
            knocked: Players::default(),
            _backend: backend,
        }
    }
    pub fn with_players(self, players: Players) -> Self {
        assert!(
            players.0.len() >= 3,
            "you need at least 3 participants to play a tournament"
        );

        let mut new_win = players;
        let mut new_lose = Players::default();
        if new_win.0.len() % 2 == 1 {
            new_win.shuffle_as_pairs(B::shuffle); // shuffle
            let (homie, guest) = (new_win.0.remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_duel = Duel {
                homie,
                guest,
                outcome: None,
            }; // create a duel
            println!("\nspecial winner duel: {w_duel}");
            let (winner, loser) = w_duel.play(B::get_outcome); // play it
            new_win.0.push(winner); // winner stays
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }
        Self {
            winner_branch: new_win.into(),
            loser_branch: new_lose.into(),
            knocked: Players::default(),
            ..self
        }
    }
    pub(crate) fn players_from_path(self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        let players = Players::load(path)?;
        Ok(self.with_players(players))
    }
    pub(crate) fn is_end(&self) -> bool {
        self.winner_branch.is_empty() && self.loser_branch.is_empty()
    }

    pub(crate) fn play_next_round(&mut self) {
        B::play_round(self);
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

// impl<B: Backend> From<Players> for Tournament<B> {
//     fn from(players: Players) -> Self {
//         assert!(
//             players.0.len() >= 3,
//             "you need at least 3 participants to play a tournament"
//         );

//         let mut new_win = players;
//         let mut new_lose = Players::default();
//         if new_win.0.len() % 2 == 1 {
//             new_win.shuffle_as_pairs(Players::shuffle); // shuffle
//             let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
//             let w_duel = Duel {
//                 homie,
//                 guest,
//                 outcome: None,
//             }; // create a duel
//             println!("\nspecial winner duel: {w_duel}");
//             let (winner, loser) = w_duel.play_cli(); // play it
//             new_win.0.push(winner); // winner stays
//             new_lose.0.push(loser); // loser get's pushed to loser branch
//         }
//         Self {
//             winner_branch: new_win.into(),
//             loser_branch: new_lose.into(),
//             knocked: Players::default(),
//             backend: Box::new(CliBackend),
//         }
//     }
// }
