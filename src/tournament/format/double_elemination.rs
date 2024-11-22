use super::*;

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
    fn play_winner_branch<B: Backend>(&mut self, shuffle: bool) -> (Players, Players) {
        let (mut next_winner_b, mut next_loser_b) = (Players::default(), Players::default());
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
        (next_winner_b, next_loser_b)
    }

    // if previous loser branch has players
    //     wait for new losers
    //     mix them into the previous loser branch
    //     execute those
    //     and execute the result again
    fn play_loser_branch<B: Backend>(&mut self, next_loser_b: &mut Players, shuffle: bool) {
        let mut prev_loser_b = std::mem::take(&mut self.loser_branch);

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
                self.knocked.0.push(loser); // loser get's knocked out of the tournament
            }
        } else {
            temp_loser_b.0 = next_loser_b.0.drain(..).collect();
        }

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
            self.knocked.0.push(loser); // loser get's knocked out of the tournament
        }
    }
}

impl<B: Backend> Format<B> for DoubleElemination {
    fn add_players(&mut self, players: Players) {
        self.winner_branch = players;
    }
    fn initial_shuffle(&mut self) {
        self.winner_branch.shuffle_as_pairs(true)
    }

    fn is_end(&self) -> bool {
        self.winner_branch.0.is_empty() && self.loser_branch.0.is_empty()
    }

    // TODO: organize to sub-functions
    fn play_round(&mut self, standard: bool) {
        let (mut next_winner_b, mut next_loser_b) = self.play_winner_branch::<B>(!standard);
        println!("\n-----------------------------");

        self.play_loser_branch::<B>(&mut next_loser_b, !standard);

        if next_winner_b.0.len() == 1 && next_loser_b.0.len() == 1 {
            // final game: only player from winner and loser branch
            let homie = next_winner_b.0.pop().unwrap();
            let guest = next_loser_b.0.pop().unwrap();
            let finals = Duel::new(homie, guest);
            println!("FINAL GAME: {finals}");
            let (winner, second) = finals.play(B::get_outcome);
            // NOTE: everyone get's to the knocked players' list,
            // as it turns into the leaderboard if reversed
            self.knocked.0.push(second);
            self.knocked.0.push(winner);
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
            self.knocked.0.push(loser); // loser get's eleminated: knocked out
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
