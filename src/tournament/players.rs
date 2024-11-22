use super::structs::*;
use std::path::Path;

#[cfg(test)]
pub mod tests;

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
    /// `shuffle` if true, order, so that every two following players make up a [`Duel`]
    pub fn shuffle_as_pairs(&mut self, shuffle: bool) {
        // shuffle to make match-making unpredictable
        if shuffle {
            fastrand::shuffle(&mut self.0);
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
            // the least similar player's index
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
    pub fn into_duels(mut self, shuffle: bool) -> Vec<Duel> {
        if self.0.is_empty() {
            return vec![];
        }

        // shuffle and sort into pairs
        self.shuffle_as_pairs(shuffle);

        // if needs bye create push it
        if self.0.len() % 2 == 1 {
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
