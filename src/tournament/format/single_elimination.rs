use super::*;

#[derive(Default, PartialEq, Eq, Clone, Debug)]
/// implemented according to wikipedia <https://en.wikipedia.org/wiki/Single-elimination_tournament>
pub struct SingleElimination {
    pub branch: Players,
    pub knocked: Players,
}

impl SingleElimination {
    pub fn new(branch: Players, knocked: Players) -> Self {
        Self { branch, knocked }
    }
}

impl Format for SingleElimination {
    fn add_players(&mut self, players: Players) {
        self.branch = players;
    }
    fn initial_shuffle(&mut self) {
        self.branch.shuffle_as_pairs();
    }

    fn is_end(&self) -> bool {
        self.branch.0.is_empty()
    }

    fn play_round(&mut self, standard: bool) {
        // winner branch of the next round
        let mut next_branch = Players::default();
        // knocked players of the next round
        let knocked = &mut self.knocked;

        let branch = std::mem::take(&mut self.branch);
        let mut branch_d = branch.into_duels(!standard);

        // get outcomes for branch duels, move contestants to other branch if necessary
        while let Some(duel) = branch_d.pop() {
            // duel isn't ready yet to be played, waiting for opponent
            if duel.guest.is_unset() {
                next_branch.0.push(duel.homie); // should get into the next-round winner branch
                continue;
            }
            println!("\nduel: {duel}");
            // play the duel, that leads us to having the result
            let (winner, loser) = duel.play();
            next_branch.0.push(winner); // winner get's to winner branch
            println!("bye-bye {loser}");
            knocked.0.push(loser); // loser get's to loser branch
        }
        println!("\n-----------------------------");

        // handle special cases on winner branch
        if next_branch.0.len() == 1 {
            self.knocked.0.push(next_branch.0.pop().unwrap());
        } else if next_branch.0.len() == 2 {
            print!("Third place duel: ");
            let mut tmp_branch = Players(vec![knocked.0.pop().unwrap(), knocked.0.pop().unwrap()]);
            let loser = Duel::handle_special(&mut tmp_branch);
            let (third, fourth) = (tmp_branch.0.pop().unwrap(), loser);
            self.knocked.0.push(fourth);
            self.knocked.0.push(third);
        } else if next_branch.0.len() % 2 == 1 {
            // not divisible by 2: we need a special pre-match: duel
            print!("\nspecial duel: ");
            let loser = Duel::handle_special(&mut next_branch);
            knocked.0.push(loser); // loser get's knocked out
        }

        // and we apply the changes by turning new branches into duels
        self.branch = next_branch;
    }

    fn print_status(&self) {
        // winner branch duels this round
        println!("--------\n\nPlayers:\n");
        for player in &self.branch.0 {
            println!("    {player}");
        }
        println!("\n-----------------------------\n\n");
    }

    fn results(self) -> Players {
        self.knocked
    }
}
