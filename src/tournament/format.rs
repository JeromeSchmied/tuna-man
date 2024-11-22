use super::{
    players::Players,
    structs::{Duel, Player},
};
use std::collections::HashMap;

pub use double_elemination::DoubleElemination;
pub use round_robin::RoundRobin;
pub use single_elemination::SingleElemination;
pub use swiss_system::SwissSystem;

pub mod double_elemination;
pub mod round_robin;
pub mod single_elemination;
pub mod swiss_system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Supported {
    SingleElemination,
    DoubleElemination,
    RoundRobin,
    SwissSystem,
}
// impl Supported {
//     pub fn to_format<B: Backend>(self) -> Box<dyn Format<B>> {
//         match self {
//             Self::SingleElemination => Box::new(format::SingleElemination::default()),
//             Self::DoubleElemination => Box::new(format::DoubleElemination::default()),
//             Self::RoundRobin => Box::new(format::RoundRobin::default()),
//             Self::SwissSystem => Box::new(format::SwissSystem::default()),
//         };
//     }
// }

/// a format in which a [`super::Tournament`] shall be made
pub trait Format {
    /// add `players` to `self`
    /// shall be used for initialization
    fn add_players(&mut self, players: Players);
    /// shuffle players
    /// should be used on initialization
    fn initial_shuffle(&mut self) {}
    /// has the tournament reached to an end?
    fn is_end(&self) -> bool;
    /// play the next round duels
    ///
    /// if `standard`, then the original order is preserved, otherwise players are shuffled after every round
    fn play_round(&mut self, standard: bool);
    /// print the actual status
    fn print_status(&self);
    /// results in reversed order
    fn results(self) -> Players;
}
