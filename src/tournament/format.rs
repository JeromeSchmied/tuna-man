use super::{
    players::Players,
    structs::{Duel, Player},
};

pub use double_elimination::DoubleElimination;
pub use round_robin::RoundRobin;
pub use single_elimination::SingleElimination;
pub use swiss_system::SwissSystem;

pub mod double_elimination;
pub mod round_robin;
pub mod single_elimination;
pub mod swiss_system;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Supported {
    SingleElimination,
    DoubleElimination,
    RoundRobin,
    SwissSystem,
}
// impl Supported {
//     pub fn to_format(self) -> Box<dyn Format> {
//         match self {
//             Self::SingleElimination => Box::new(SingleElimination::default()),
//             Self::DoubleElimination => Box::new(DoubleElimination::default()),
//             Self::RoundRobin => Box::new(RoundRobin::default()),
//             Self::SwissSystem => Box::new(SwissSystem::default()),
//         }
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
