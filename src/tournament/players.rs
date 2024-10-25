use super::structs::*;
use std::path::Path;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct Players(pub(crate) Vec<Player>);

impl Players {
    pub(crate) fn load(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        let players = reader
            .deserialize()
            .flat_map(|x| x.inspect_err(|e| eprintln!("error: {e:#?}")))
            .collect();
        Ok(Self(players))
    }
    pub(crate) fn save(self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let mut writer = csv::Writer::from_path(path)?;
        self.0.iter().try_for_each(|p| writer.serialize(p))?;
        writer.flush()
    }
    pub(crate) fn shuffle_as_pairs(&mut self, shuffle: impl FnOnce(&mut Self)) {
        if self.0.is_empty() {
            return;
        }
        shuffle(self);
        let mut as_pairs = Vec::new();
        while self.0.len() > 1 {
            let cnt = self.0.swap_remove(0);
            let idx = Self::diff_list(&self.0, &cnt)
                .expect("possibly number of players isn't divisible by two");
            as_pairs.push(cnt);
            as_pairs.push(self.0.swap_remove(idx));
        }
        if self.0.len() == 1 {
            as_pairs.push(self.0.pop().unwrap());
        }
        self.0 = as_pairs;
    }
    pub fn into_vec_duel(self, shuffle: impl FnOnce(&mut Self)) -> Vec<Duel> {
        if self.0.len() == 1 {
            let halfset_duel = Duel {
                homie: self.0[0].clone(),
                guest: Player::default(),
                outcome: None,
            };
            return vec![halfset_duel];
        }
        self.transform(shuffle)
            .into_iter()
            .map(std::convert::Into::into)
            .collect()
    }
    fn transform(self, shuffle: impl FnOnce(&mut Self)) -> Vec<(Player, Player)> {
        let mut players = self;
        assert_eq!(players.0.len() % 2, 0);
        players.shuffle_as_pairs(shuffle);
        let mut res = Vec::new();
        while !players.0.is_empty() {
            let cnt = players.0.swap_remove(0);
            let next = players.0.swap_remove(0);
            res.push((cnt, next));
        }
        res
    }
    /// # Usage
    ///
    /// returns index
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
        if hay.class.is_none() {
            return Some(0);
        }
        // index, value
        let mut max: (Option<usize>, u8) = (None, 0);
        for (i, p) in haystack.iter().enumerate() {
            let diff = if hay.class == p.class {
                // same class
                1
            } else if hay.class.unwrap().grade == p.class.unwrap().grade {
                // same grade
                2
            } else if hay.class.unwrap().id == p.class.unwrap().id {
                // same class id
                3
            } else {
                4
            };
            if diff > max.1 {
                max.1 = diff;
                max.0 = Some(i);
                // found one that's already highest value, use it
                if max.1 == 4 {
                    break;
                }
            }
        }
        // dbg!(max);
        max.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

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
    fn basic_shuffle_pairs() {
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
        players.shuffle_as_pairs(|_| {});
        let exp = Players(vec![
            np("Central Mite"),
            np("Casual Ptarmigan"),
            np("Expectant Wolfhound"),
            np("Profound Ponytail"),
            np("Inviting Pheasant"),
            np("Usable Bengal"),
            np("Droll Jaguar"),
            np("Exotic Skunk"),
            np("Relative Wrasse"),
        ]);
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
        players.shuffle_as_pairs(|_| {});
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
