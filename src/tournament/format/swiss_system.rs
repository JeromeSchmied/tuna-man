use super::*;

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Swiss-system_tournament>
pub struct SwissSystem {}

impl SwissSystem {
    //     pub fn new(branch: Vec<Duel>, knocked: Players) -> Self {
    //         Self { branch, knocked }
    //     }
}

impl Format for SwissSystem {
    fn add_players(&mut self, players: Players) {
        todo!()
    }

    fn is_end(&self) -> bool {
        todo!()
    }

    fn play_round(&mut self, _: bool) {
        todo!()
    }

    fn print_status(&self) {
        todo!()
    }

    fn results(self) -> Players {
        todo!()
    }
}
