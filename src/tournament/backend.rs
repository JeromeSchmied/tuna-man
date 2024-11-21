use super::{format::Format, structs::Duel, Tournament};
use std::io::Write;

/// Internal features necessary for retrieving information about a [`Tournament`]
pub trait Backend {
    /// how to retrieve outcome of a [`Duel`]
    fn get_outcome(duel: Duel) -> Result<Duel, ()>;

    /// play a round of a [`Tournament`]
    /// double-knockout for now
    // TODO: support more formats
    fn play_round<B: Backend, F: Format<B>>(tournament: &mut Tournament<B, F>, standard: bool)
    where
        Self: Sized,
    {
        tournament.format.play_round(standard);
    }
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Test;
impl Backend for Test {
    fn get_outcome(duel: Duel) -> Result<Duel, ()> {
        Ok(duel.clone().with_outcome(Some(true)))
    }
}
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cli;
impl Backend for Cli {
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
