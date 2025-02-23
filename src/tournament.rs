use format::Format;
use players::Players;
use std::path::Path;

/// # the format of the tournament
///
/// ## available formats:
///
/// - [x] [single-knockout](https://en.wikipedia.org/wiki/Single-elimination_tournament)
/// - [x] [double-knockout](https://en.wikipedia.org/wiki/Double-elimination_tournament)
/// - [x] [round-robin](https://en.wikipedia.org/wiki/Round-robin_tournament)
/// - [ ] [swiss-system](https://en.wikipedia.org/wiki/Swiss-system_tournament)
pub mod format;
/// dealing with a bunch of players
mod players;
/// building block structs
mod structs;
#[cfg(test)]
pub mod tests;

/// The whole [`Tournament`] with all the [`Players`] and [`Duel`]s
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Tournament<F: Format> {
    format: F,
}

impl<F: Format> Tournament<F> {
    pub fn new(format: F) -> Self {
        Self { format }
    }
    /// execute the Tournament with options from `args`
    pub fn execute(self, args: crate::args::Args) -> std::io::Result<()> {
        self.players_from_path(&args.file)?.run(args);
        Ok(())
    }
    /// run the whole Tournament
    pub fn run(mut self, args: crate::args::Args) {
        let no_shuffle = args.shuffle.never() || args.shuffle.initially();
        if args.shuffle.initially() || args.shuffle.always() {
            self.format.initial_shuffle();
        }

        // number of rounds
        let mut round = 0;

        // run till we've got all the results
        while !self.is_end() {
            // winner branch duels this round
            println!("\n\n\n\nRound {round}.\n");
            self.format.print_status();
            self.play_next_round(no_shuffle);

            round += 1;
        }

        let mut knocked = self.format.results();
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
    /// `self` but with `players`
    pub fn with_players(mut self, players: Players) -> Self {
        assert!(
            players.0.len() >= 3,
            "you need at least 3 participants to play a tournament"
        );
        self.format.add_players(players);

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
    pub fn play_next_round(&mut self, standard: bool) {
        self.format.play_round(standard);
    }
    // pub fn execute(
    //     &mut self,
    //     term: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
    // ) -> std::io::Result<()> {
    //     loop {
    //         term.draw(|f| self.ui(f))?;
    //         if let Event::Key(key) = event::read()? {
    //             if key.kind != KeyEventKind::Press {
    //                 continue;
    //             }
    //             match key.code {
    //                 KeyCode::Char('q') | KeyCode::Esc => break,
    //                 KeyCode::Char('r') => restart(),
    //                 KeyCode::Char('l') | KeyCode::Right => cursor.right(),
    //                 KeyCode::Char('h') | KeyCode::Left => cursor.left(),
    //                 KeyCode::Char('j') | KeyCode::Down => cursor.down(),
    //                 KeyCode::Char('k') | KeyCode::Up => cursor.up(),
    //                 _ => {}
    //             }
    //         } else {
    //         }
    //     }
    //     Ok(())
    // }
}
