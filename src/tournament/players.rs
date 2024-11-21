use super::structs::*;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Players(pub Vec<Player>);

impl Players {
    /// load players from file at `path`
    pub fn load(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        let players = reader
            .deserialize()
            .flat_map(|x| x.inspect_err(|e| eprintln!("error: {e:#?}")))
            .collect();
        Ok(Self(players))
    }
    /// save `self` to file at `path`
    pub fn save(self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut writer = csv::Writer::from_path(path)?;
        self.0.iter().try_for_each(|p| writer.serialize(p))?;
        writer.flush()
    }
    /// shuffle in a way, that every two following players make up a [`Duel`]
    pub fn shuffle_as_pairs(&mut self, shuffle: Option<impl FnOnce(&mut Self)>) {
        // shuffle to make match-making unpredictable
        if let Some(shuffle) = shuffle {
            shuffle(self);
        }
        if self.0.first().is_some_and(|p| p.class.is_none()) {
            // if no classes present, no need for diff-list
            return;
        }
        // here'll be the players ordered as pairs
        let mut as_pairs = Vec::new();
        // 2 players always needed to make up a duel
        while self.0.len() > 1 {
            // current player
            let cnt = self.0.swap_remove(0);
            // the least similar player
            let idx = Self::diff_list(&self.0, &cnt)
                .expect("possibly number of players isn't divisible by two");
            as_pairs.push(cnt); // first the current player
            as_pairs.push(self.0.swap_remove(idx)); // then the selected one
        }
        // someone's remained, it's pushed to end
        as_pairs.append(&mut self.0);
        self.0 = as_pairs; // apply changes
    }
    /// convert `self` into [`Duel`]s
    pub fn into_duels(mut self, shuffle: Option<impl FnOnce(&mut Self)>) -> Vec<Duel> {
        if self.0.is_empty() {
            return vec![];
        }
        let contains_bye = self.0.contains(&Player::default());
        if contains_bye {
            self.0.retain(|p| !p.is_unset());
        }

        // TODO: add bye here
        // only works with even number of players
        // assert_eq!(self.0.len() % 2, 0);

        // shuffle and sort into pairs
        self.shuffle_as_pairs(shuffle);
        // if needs bye create push it
        if contains_bye || self.0.len() % 2 == 1 {
            self.0.push(Player::default());
        }

        self.0
            .rchunks_exact_mut(2)
            .map(|c| Duel::new(std::mem::take(&mut c[0]), std::mem::take(&mut c[1])))
            .collect()
    }
    /// # Usage
    ///
    /// find player with highest difference from `hay` in `haystack`
    /// returns index for `haystack`
    ///
    /// # Implementation
    ///
    /// greedy.
    ///
    /// calculate `diff_list`, move the one with highest value from players to results
    /// calculation: least similar class:
    /// 1 same class: grade+id
    /// 2 same id
    /// 3 same grade
    /// 4 nothing in common (based on known things) cool!
    fn diff_list(haystack: &[Player], hay: &Player) -> Option<usize> {
        // no factor of difference: first one will be just fine
        if hay.class.is_none() {
            return Some(0);
        }
        // max (index, value)
        let mut max: (Option<usize>, u8) = (None, 0);
        // calculate difference for all player
        // will break soon if highest difference factor is found
        for (i, p) in haystack.iter().enumerate() {
            let diff = if hay.class == p.class {
                1
            } else if hay.class.unwrap().grade == p.class.unwrap().grade {
                2
            } else if hay.class.unwrap().id == p.class.unwrap().id {
                3
            } else {
                4
            };
            // update max if needed
            if diff > max.1 {
                max.1 = diff; // value
                max.0 = Some(i); // index
                if max.1 == 4 {
                    // found one that's already highest value, use it
                    break;
                }
            }
        }
        max.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::{super::backend, *};

    const SHUFFLE: Option<fn(&mut Players)> = Some(<backend::Test as backend::Backend>::shuffle);

    pub fn load_players() -> Players {
        Players::load("data.csv").unwrap()
    }

    #[test]
    #[should_panic]
    fn antiload() {
        Players::load("Low for the sake of environment.bacilus").unwrap();
    }

    pub fn nu_p(name: &str, grade: u8, id: char) -> Player {
        Player::new(name, Class::new(grade, id))
    }
    fn np(name: &str) -> Player {
        Player {
            name: name.into(),
            class: None,
        }
    }

    #[test]
    fn load() {
        assert_eq!(
            Players(vec![
                nu_p("Central Mite", 10, 'D'),
                nu_p("Relative Wrasse", 10, 'C'),
                nu_p("Exotic Skunk", 00, 'A'),
                nu_p("Droll Jaguar", 12, 'C'),
                nu_p("Usable Bengal", 9, 'C'),
                nu_p("Inviting Pheasant", 12, 'B'),
                nu_p("Profound Ponytail", 00, 'B'),
                nu_p("Expectant Wolfhound", 9, 'D'),
                nu_p("Casual Ptarmigan", 11, 'B')
            ]),
            load_players()
        );
    }

    fn get_dl(ps: &mut Players, idx: usize) -> Option<usize> {
        let p = ps.0.remove(idx);
        let res = Players::diff_list(&ps.0, &p);
        ps.0.insert(idx, p);
        res
    }
    #[test]
    fn basic_diff_list() {
        let mut players = load_players();
        let result = (0..players.0.len())
            .map(|i| get_dl(&mut players, i).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![1, 1, 0, 0, 0, 0, 0, 1, 0];
        assert_eq!(result, expected);
    }

    #[test]
    fn more_diff_list() {
        let mut players = load_players();

        let mut shrex = || -> (Player, Player) {
            let homie = players.0.remove(0);
            let Some(guest_idx) = Players::diff_list(&players.0, &homie) else {
                return (homie, Player::default());
            };
            (homie, players.0.remove(guest_idx))
        };

        assert_eq!(
            shrex(),
            (nu_p("Central Mite", 10, 'D'), nu_p("Exotic Skunk", 00, 'A'))
        );
        assert_eq!(
            shrex(),
            (
                nu_p("Relative Wrasse", 10, 'C'),
                nu_p("Inviting Pheasant", 12, 'B'),
            )
        );
        assert_eq!(
            shrex(),
            (
                nu_p("Droll Jaguar", 12, 'C'),
                nu_p("Profound Ponytail", 00, 'B'),
            )
        );
        assert_eq!(
            shrex(),
            (
                nu_p("Usable Bengal", 9, 'C'),
                nu_p("Casual Ptarmigan", 11, 'B')
            )
        );
        assert_eq!(
            shrex(),
            (nu_p("Expectant Wolfhound", 9, 'D'), Player::default())
        );
    }
    #[test]
    fn no_shuffle_pairs() {
        let mut players = load_players();
        for player in players.0.iter_mut() {
            player.class = None;
        }
        let exp = Players(vec![
            np("Central Mite"),
            np("Relative Wrasse"),
            np("Exotic Skunk"),
            np("Droll Jaguar"),
            np("Usable Bengal"),
            np("Inviting Pheasant"),
            np("Profound Ponytail"),
            np("Expectant Wolfhound"),
            np("Casual Ptarmigan"),
        ]);
        assert_eq!(exp, players);
        players.shuffle_as_pairs(SHUFFLE);
        assert_eq!(exp, players);
    }
    #[test]
    fn more_shuffle_pairs() {
        let mut players = load_players();
        let exp = Players(vec![
            nu_p("Central Mite", 10, 'D'),
            nu_p("Relative Wrasse", 10, 'C'),
            nu_p("Exotic Skunk", 00, 'A'),
            nu_p("Droll Jaguar", 12, 'C'),
            nu_p("Usable Bengal", 9, 'C'),
            nu_p("Inviting Pheasant", 12, 'B'),
            nu_p("Profound Ponytail", 00, 'B'),
            nu_p("Expectant Wolfhound", 9, 'D'),
            nu_p("Casual Ptarmigan", 11, 'B'),
        ]);
        assert_eq!(exp, players);
        players.shuffle_as_pairs(SHUFFLE);
        let exp = Players(vec![
            nu_p("Central Mite", 10, 'D'),
            nu_p("Casual Ptarmigan", 11, 'B'),
            nu_p("Expectant Wolfhound", 9, 'D'),
            nu_p("Profound Ponytail", 00, 'B'),
            nu_p("Inviting Pheasant", 12, 'B'),
            nu_p("Usable Bengal", 9, 'C'),
            nu_p("Droll Jaguar", 12, 'C'),
            nu_p("Exotic Skunk", 00, 'A'),
            nu_p("Relative Wrasse", 10, 'C'),
        ]);
        assert_eq!(exp, players);
    }
}
