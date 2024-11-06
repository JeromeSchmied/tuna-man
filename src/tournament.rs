use backend::Backend;
use format::Format;
use players::Players;
use std::path::Path;

/// how we interact with the user
pub mod backend;
/// # the format of the tournament
///
/// ## available formats:
///
/// - [x] single-knockout
/// - [x] double-knockout
/// - [ ] round-robin
/// - [ ] swiss-system
pub mod format;
/// dealing with a bunch of players
mod players;
/// building block structs
mod structs;
#[cfg(test)]
pub mod tests;

/// The whole [`Tournament`] with all the [`Players`] and [`Duel`]s
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Tournament<B: Backend, F: Format<B>> {
    format: F,
    /// the [`backend::Backend`] to use
    _backend: B,
}

impl<B: Backend, F: Format<B>> Tournament<B, F> {
    // pub fn with_tables(self, tables: &[Table]) -> Self {
    //     Self {
    //         tables: tables.into(),
    //         ..self
    //     }
    // }
    pub fn run(mut self) {
        // number of rounds
        let mut round = 0;

        // run till we've got all the results
        while !self.is_end() {
            // winner branch duels this round
            println!("\n\n\n\nRound {round}.\n");
            self.print_status();
            self.play_next_round();

            round += 1;
        }

        let mut knocked = self.knocked();
        // printing results
        println!("\nTournament ended in {round} rounds, Results:");
        println!("\n\nPODIUM\n------\n");
        println!("Winner: {}", knocked.0.pop().unwrap());
        println!("Second place: {}", knocked.0.pop().unwrap());
        println!("Third place: {}", knocked.0.pop().unwrap());
        println!("\nrunner-ups\n");
        for (place, player) in knocked.0.iter().rev().enumerate() {
            println!("{}. place: {player}", place + 4);
        }
    }
    pub fn print_status(&self) {
        self.format.print_status();
    }
    pub fn new(backend: B, format: F) -> Self {
        Self {
            _backend: backend,
            format,
        }
    }
    pub fn knocked(self) -> Players {
        self.format.knocked()
    }
    /// `self` but with `players`
    pub fn with_players(mut self, players: Players) -> Self {
        assert!(
            players.0.len() >= 3,
            "you need at least 3 participants to play a tournament"
        );
        self.format.from_players(players);

        self
    }
    /// add players to `self` read from file at `path`
    pub fn players_from_path(self, path: impl AsRef<Path>) -> std::io::Result<Self> {
        let players = Players::load(path)?;
        Ok(self.with_players(players))
    }
    /// `self` is ended, we've got all the results
    pub fn is_end(&self) -> bool {
        self.format.is_end()
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
