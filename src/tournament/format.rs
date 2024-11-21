use super::{
    backend::Backend,
    players::Players,
    structs::{Duel, Player},
};
use std::collections::HashMap;

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
//             Supported::SingleElemination => Box::new(SingleElemination::default()),
//             Supported::DoubleElemination => Box::new(DoubleElemination::default()),
//             Supported::RoundRobin => todo!(),
//             Supported::SwissSystem => todo!(),
//         }
//     }
// }

/// a format in which a [`super::Tournament`] shall be made
pub trait Format<B: Backend> {
    /// add `players` to `self`
    /// shall be used on initialization
    ///
    /// if `standard`, then the original order is preserved, otherwise players are shuffled
    fn add_players(&mut self, players: Players, standard: bool);
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

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Double-elimination_tournament>
pub struct DoubleElemination {
    winner_branch: Players,
    loser_branch: Players,
    knocked: Players,
}
impl DoubleElemination {
    pub fn new(winner_branch: Players, loser_branch: Players, knocked: Players) -> Self {
        Self {
            winner_branch,
            loser_branch,
            knocked,
        }
    }
}
impl<B: Backend> Format<B> for DoubleElemination {
    fn add_players(&mut self, players: Players, standard: bool) {
        let mut new_win = players;
        if !standard {
            new_win.shuffle_as_pairs(Some(B::shuffle));
        }
        let mut new_lose = Players::default();
        if new_win.0.len() % 2 == 1 {
            println!("\nspecial winner duel: ");
            let loser = Duel::handle_special::<B>(&mut new_win);
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }
        *self = Self {
            winner_branch: new_win,
            loser_branch: new_lose,
            knocked: Players::default(),
        };
    }

    fn is_end(&self) -> bool {
        self.winner_branch.0.is_empty() && self.loser_branch.0.is_empty()
    }

    // TODO: organize to sub-functions
    fn play_round(&mut self, standard: bool) {
        dbg!(&self);
        // winner branch of the next round
        let mut next_winner_b = Players::default();
        // loser branch of the next round
        let mut next_loser_b = Players::default();
        // knocked players of the next round
        let knocked = &mut self.knocked;

        let shuffle = if standard { None } else { Some(B::shuffle) };
        let mut winner_b = std::mem::take(&mut self.winner_branch).into_duels(shuffle);

        // get outcomes for winner branch duels, move contestants to other branch if necessary
        while let Some(w_duel) = winner_b.pop() {
            // duel isn't ready yet to be played, waiting for opponent
            if w_duel.guest.is_unset() {
                next_winner_b.0.push(w_duel.homie); // should get into the next-round winner branch
                continue;
            }
            println!("\nwinner duel: {w_duel}");
            // play the duel, that leads us to having the result
            let (winner, loser) = w_duel.play(B::get_outcome);
            next_winner_b.0.push(winner); // winner get's to winner branch
            next_loser_b.0.push(loser); // loser get's to loser branch
        }
        println!("\n-----------------------------");
        // if previous loser branch has players
        //     wait for new losers
        //     mix them into the previous loser branch
        //     execute those
        //     and execute the result again
        // TODO: careful with byes
        {
            let mut prev_loser_b = std::mem::take(&mut self.loser_branch);
            dbg!(&next_winner_b, &prev_loser_b, &next_loser_b,);

            let mut temp_loser_b = Players::default();
            if !prev_loser_b.0.is_empty() {
                // insert new losers into prev losers
                {
                    let mut i = 1;
                    while i <= prev_loser_b.0.len() && !next_loser_b.0.is_empty() {
                        prev_loser_b.0.insert(i, next_loser_b.0.remove(0));
                        i += 2;
                    }
                }

                let mut first_loser_d = prev_loser_b.into_duels(shuffle);
                // get outcomes for loser branch duels, move contestants to other branch if necessary
                while let Some(l_duel) = first_loser_d.pop() {
                    // duel isn't ready yet to be played, waiting for opponent
                    if l_duel.guest.is_unset() {
                        temp_loser_b.0.push(l_duel.homie); // should get into the next-round loser branch
                        continue;
                    }
                    println!("\nloser duel: {l_duel}");
                    // play the duel, that leads us to having the result
                    let (winner, loser) = l_duel.play(B::get_outcome);
                    temp_loser_b.0.push(winner); // winner get's to loser branch
                    println!("bye-bye {loser}");
                    knocked.0.push(loser); // loser get's knocked out of the tournament
                }
            } else {
                temp_loser_b.0 = next_loser_b.0.drain(..).collect();
            }

            dbg!(&next_winner_b, &temp_loser_b, &next_loser_b);
            if temp_loser_b.0.len() % 2 == 1 {
                dbg!("bye needed");
                temp_loser_b.0.push(Player::default());
            }
            assert_ne!(temp_loser_b.0.len() % 2, 1, "scheiß, so geht's ned!");
            let mut second_loser_d = temp_loser_b.into_duels(shuffle);
            // get outcomes for loser branch duels, move contestants to other branch if necessary
            while let Some(l_duel) = second_loser_d.pop() {
                // duel isn't ready yet to be played, waiting for opponent
                if l_duel.guest.is_unset() {
                    next_loser_b.0.push(l_duel.homie); // should get into the next-round loser branch
                    continue;
                }
                println!("\nsecond-round loser duel: {l_duel}");
                // play the duel, that leads us to having the result
                let (winner, loser) = l_duel.play(B::get_outcome);
                next_loser_b.0.push(winner); // winner get's to loser branch
                println!("bye-bye {loser}");
                knocked.0.push(loser); // loser get's knocked out of the tournament
            }
        }
        dbg!(&next_winner_b, &next_loser_b, &knocked);
        if next_winner_b.0.len() == 1 && next_loser_b.0.len() == 1 {
            // final game: only player from winner and loser branch
            let homie = next_winner_b.0.pop().unwrap();
            let guest = next_loser_b.0.pop().unwrap();
            let finals = Duel::new(homie, guest);
            println!("FINAL GAME: {finals}");
            let (winner, second) = finals.play(B::get_outcome);
            // NOTE: everyone get's to the knocked players' list,
            // as it turns into the leaderboard if reversed
            knocked.0.push(second);
            knocked.0.push(winner);
        }

        // handle special cases on winner branch
        if next_winner_b.0.len() == 1 {
            println!(
                "soon final: only winner branch remainder: {}",
                next_winner_b.0[0]
            );
        } else if next_winner_b.0.len() % 2 == 1 {
            // not divisible by 2: we need a special pre-match: duel
            print!("\nspecial winner duel: ");
            let loser = Duel::handle_special::<B>(&mut next_winner_b);
            next_loser_b.0.push(loser); // loser get's pushed to loser branch
        }

        // handle special cases on loser branch
        if next_loser_b.0.len() == 1 {
            println!("FINAL SOON");
            // // final game: only player from winner and loser branch
            // let homie = next_winner_b.0.pop().unwrap();
            // let guest = next_loser_b.0.pop().unwrap();
            // let finals = Duel::new(homie, guest);
            // println!("FINAL GAME: {finals}");
            // let (winner, second) = finals.play(B::get_outcome);
            // // NOTE: everyone get's to the knocked players' list,
            // // as it turns into the leaderboard if reversed
            // knocked.0.push(second);
            // knocked.0.push(winner);
        } else if next_loser_b.0.len() % 2 == 1 {
            // not divisible by 2: we need a special pre-match: duel
            println!("\nspecial loser duel: ");
            let loser = Duel::handle_special::<B>(&mut next_loser_b);
            println!("bye-bye {loser}");
            knocked.0.push(loser); // loser get's eleminated: knocked out
        }
        // and we apply the changes by turning new branches into duels
        self.winner_branch = next_winner_b;
        self.loser_branch = next_loser_b;
    }

    fn print_status(&self) {
        // winner branch duels this round
        println!("--------\n\nWinner branch players:\n");
        for w_player in &self.winner_branch.0 {
            println!("    {w_player}");
        }
        // loser branch duels this round
        println!("\n-----------------------------\n\nLosing branch players:\n");
        for l_player in &self.loser_branch.0 {
            println!("    {l_player}");
        }
        println!("\n-----------------------------\n\n");
    }

    fn results(self) -> Players {
        self.knocked
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Single-elimination_tournament>
pub struct SingleElemination {
    branch: Vec<Duel>,
    knocked: Players,
}
impl SingleElemination {
    pub fn new(branch: Vec<Duel>, knocked: Players) -> Self {
        Self { branch, knocked }
    }
}
impl<B: Backend> Format<B> for SingleElemination {
    fn add_players(&mut self, players: Players, standard: bool) {
        let mut branch = players;
        let mut knocked = Players::default();
        if branch.0.len() % 2 == 1 {
            println!("\nspecial duel: ");
            let loser = Duel::handle_special::<B>(&mut branch);
            knocked.0.push(loser); // loser get's pushed to loser branch
        }
        let shuffle = if standard { None } else { Some(B::shuffle) };
        *self = Self {
            branch: branch.into_duels(shuffle),
            knocked,
        };
    }

    fn is_end(&self) -> bool {
        self.branch.is_empty()
    }

    fn play_round(&mut self, standard: bool) {
        // winner branch of the next round
        let mut new_branch = Players::default();
        // knocked players of the next round
        let knocked = &mut self.knocked;

        // get outcomes for branch duels, move contestants to other branch if necessary
        while let Some(duel) = self.branch.pop() {
            // duel isn't ready yet to be played, waiting for opponent
            if duel.guest.is_unset() {
                new_branch.0.push(duel.homie); // should get into the next-round winner branch
                continue;
            }
            println!("\nduel: {duel}");
            // play the duel, that leads us to having the result
            let (winner, loser) = duel.play(B::get_outcome);
            new_branch.0.push(winner); // winner get's to winner branch
            println!("bye-bye {loser}");
            knocked.0.push(loser); // loser get's to loser branch
        }
        println!("\n-----------------------------");

        // handle special cases on winner branch
        if new_branch.0.len() == 1 {
            self.knocked.0.push(new_branch.0.pop().unwrap());
        } else if new_branch.0.len() == 2 {
            print!("Third place duel: ");
            let mut tmp_branch = Players(vec![knocked.0.pop().unwrap(), knocked.0.pop().unwrap()]);
            let loser = Duel::handle_special::<B>(&mut tmp_branch);
            let (third, fourth) = (tmp_branch.0.pop().unwrap(), loser);
            self.knocked.0.push(fourth);
            self.knocked.0.push(third);
        } else if new_branch.0.len() % 2 == 1 {
            // not divisible by 2: we need a special pre-match: duel
            print!("\nspecial duel: ");
            let loser = Duel::handle_special::<B>(&mut new_branch);
            knocked.0.push(loser); // loser get's knocked out
        }

        let shuffle = if standard { None } else { Some(B::shuffle) };
        // and we apply the changes by turning new branches into duels
        self.branch = new_branch.into_duels(shuffle);
    }

    fn print_status(&self) {
        // winner branch duels this round
        println!("--------\n\nDuels:\n");
        for duel in &self.branch {
            println!("    {duel}");
        }
        println!("\n-----------------------------\n\n");
    }

    fn results(self) -> Players {
        self.knocked
    }
}

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Round-robin_tournament>
pub struct RoundRobin {
    /// currently relevant [`Duel`]s to be carried out
    duels: Vec<Duel>,
    /// all the participating [`Players`]
    players: Players,
    /// points of the `players`
    points: HashMap<Player, u8>,
    /// the number of `round`s already executed
    round: usize,
}
impl RoundRobin {
    /// number of [`Self::players`]
    fn len(&self) -> usize {
        self.players.0.len()
    }
    /// update the [`Self::duels`], so in the upcoming round [`Player`]s play against other ones as well
    /// circle-method, implemented according to wikipedia <https://en.wikipedia.org/wiki/Round-robin_tournament#Circle_method>
    fn update_duels(&mut self) {
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
        // apply changes
        self.duels = duels;
    }
    pub fn new(
        duels: Vec<Duel>,
        players: Players,
        points: HashMap<Player, u8>,
        round: usize,
    ) -> Self {
        Self {
            duels,
            players,
            points,
            round,
        }
    }
}
impl<B: Backend> Format<B> for RoundRobin {
    fn add_players(&mut self, players: Players, _: bool) {
        // simply apply players
        self.players = players;

        // odd number of players
        if self.len() % 2 == 1 {
            // add ghost player: bye
            self.players.0.push(Player::default());
        }

        // set every player's points to 0
        let points = self
            .players
            .0
            .clone()
            .into_iter()
            .map(|p| (p, 0u8))
            .collect::<HashMap<_, _>>();
        // apply points
        self.points = points;

        // init duels
        self.update_duels();
    }

    fn is_end(&self) -> bool {
        // every player played against every player
        self.round == self.len()
    }

    fn play_round(&mut self, _: bool) {
        // execute duels: get outcomes
        for duel in &self.duels {
            println!("\n{duel}");
            // ignore duel if any players are ghosts
            if duel.homie.is_unset() || duel.guest.is_unset() {
                continue;
            }
            // execute duel
            let (winner, _loser) = duel.clone().play(B::get_outcome);
            // winner get's a point
            self.points.entry(winner).and_modify(|p| *p += 1);
        }
        // update the duels for the upcoming round
        self.update_duels();
        // another round is executed
        self.round += 1;
    }

    fn print_status(&self) {
        println!("\n\nPOINTS:\n");
        for player in &self.players.0 {
            println!("    {player}: {}", self.points[player]);
        }
        println!("\n\n-------------------\n\nDUELS:\n");
        for duel in &self.duels {
            println!("    {duel}");
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

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Swiss-system_tournament>
pub struct SwissSystem {}
impl SwissSystem {
    //     pub fn new(branch: Vec<Duel>, knocked: Players) -> Self {
    //         Self { branch, knocked }
    //     }
}
impl<B: Backend> Format<B> for SwissSystem {
    fn add_players(&mut self, players: Players, _: bool) {
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
