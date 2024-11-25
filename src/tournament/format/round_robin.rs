use super::*;
use std::collections::HashMap;

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Round-robin_tournament>
pub struct RoundRobin {
    /// all the participating [`Players`]
    pub players: Players,
    /// points of the `players`
    pub points: HashMap<Player, u8>,
    /// the number of `round`s already executed
    pub round: usize,
}

impl RoundRobin {
    /// number of [`Self::players`]
    pub fn len(&self) -> usize {
        self.players.0.len()
    }
    /// update the [`Self::duels`], so in the upcoming round [`Player`]s play against other ones as well
    /// circle-method, implemented according to wikipedia <https://en.wikipedia.org/wiki/Round-robin_tournament#Circle_method>
    pub fn gen_duels(&mut self) -> Vec<Duel> {
        // the indexed order of duels
        let mut duel_idxs = (1..self.len()).collect::<Vec<_>>();
        duel_idxs.rotate_right(self.round);
        duel_idxs.insert(0, 0);

        let mut duels = vec![];
        while let Some(guest) = duel_idxs.pop() {
            // index of the first one
            let homie = duel_idxs.remove(0);
            // the actual players themselves
            let (homie, guest) = (
                self.players.0.get(homie).unwrap().clone(),
                self.players.0.get(guest).unwrap().clone(),
            );
            duels.push(Duel::new(homie, guest));
        }
        duels
    }
    pub fn new(players: Players, points: HashMap<Player, u8>, round: usize) -> Self {
        Self {
            players,
            points,
            round,
        }
    }
}

impl Format for RoundRobin {
    fn add_players(&mut self, mut players: Players) {
        // odd number of players
        if players.0.len() % 2 == 1 {
            // add ghost player: bye
            players.0.push(Player::default());
        }
        // simply apply players
        self.players = players.clone();

        // set every player's points to 0
        self.points = players.0.into_iter().map(|p| (p, 0)).collect();
    }

    fn is_end(&self) -> bool {
        // every player played against every player
        self.round == self.len() - 1
    }

    fn play_round(&mut self, _: bool) {
        // execute duels: get outcomes
        for duel in self.gen_duels() {
            println!("\n{duel}");
            // ignore duel if any players are ghosts
            if duel.homie.is_unset() || duel.guest.is_unset() {
                continue;
            }
            // execute duel
            let (winner, _loser) = duel.clone().play();
            // winner get's a point
            self.points.entry(winner).and_modify(|p| *p += 1);
        }
        // another round is executed
        self.round += 1;
    }

    fn print_status(&self) {
        println!("\n\nPOINTS:\n");
        for player in &self.players.0 {
            println!("    {player}: {}", self.points[player]);
        }
        println!("\n\n\n");
    }

    fn results(self) -> Players {
        // hashmap -> vec
        let mut result: Vec<_> = self.points.into_iter().collect();
        // sorted by points
        result.sort_by(|x, y| x.1.cmp(&y.1));
        // extract players
        let result: Vec<_> = result.into_iter().map(|(player, _points)| player).collect();
        // into Players
        Players(result)
    }
}
