use backend::Backend;
use players::Players;
use std::path::Path;
use structs::Duel;

/// how we interact with the user
pub mod backend;
/// dealing with a bunch of players
mod players;
/// building block structs
mod structs;
#[cfg(test)]
pub mod tests;

/// The whole [`Tournament`] with all the [`Players`] and [`Duel`]s
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Tournament<B: Backend> {
    /// [`Duel`]s on the winner branch
    pub winner_branch: Vec<Duel>,
    /// [`Duel`]s on the loser branch
    pub loser_branch: Vec<Duel>,
    /// [`Players`] who were knocked from the [`Tournament`]
    pub knocked: Players,
    /// the [`backend::Backend`] to use
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
    /// `self` but with `players`
    pub fn with_players(self, players: Players) -> Self {
        assert!(
            players.0.len() >= 3,
            "you need at least 3 participants to play a tournament"
        );

        let mut new_win = players;
        let mut new_lose = Players::default();
        if new_win.0.len() % 2 == 1 {
            println!("\nspecial winner duel: ");
            let loser = Duel::handle_special::<B>(&mut new_win);
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }
        Self {
            winner_branch: new_win.into_duels(B::shuffle),
            loser_branch: new_lose.into_duels(B::shuffle),
            knocked: Players::default(),
            ..self
        }
    }
    /// add players to `self` read from file at `path`
    pub fn players_from_path(self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        let players = Players::load(path)?;
        Ok(self.with_players(players))
    }
    /// `self` is ended, we've got all the results
    pub fn is_end(&self) -> bool {
        self.winner_branch.is_empty() && self.loser_branch.is_empty()
    }
    /// play the next round
    pub fn play_next_round(&mut self) {
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
