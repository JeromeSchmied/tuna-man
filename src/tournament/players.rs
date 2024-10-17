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
    pub(crate) fn shuffle_as_pairs(&mut self) {
        if self.0.is_empty() {
            return;
        }
        fastrand::shuffle(&mut self.0);
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
    fn transform(self) -> Vec<(Player, Player)> {
        let mut players = self;
        assert_eq!(players.0.len() % 2, 0);
        players.shuffle_as_pairs();
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

impl From<Players> for Vec<Match> {
    fn from(players: Players) -> Self {
        if players.0.len() == 1 {
            let halfset_match = Match {
                homie: players.0[0].clone(),
                guest: Player::default(),
                outcome: None,
            };
            return vec![halfset_match];
        }
        players
            .transform()
            .into_iter()
            .map(std::convert::Into::into)
            .collect()
    }
}
