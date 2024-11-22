use super::players::Players;
use serde::{Deserialize, Serialize};
#[cfg(not(test))]
use std::io::Write;

/// a player/contestant/participant/team of a [`super::Tournament`]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, Hash)]
pub struct Player {
    /// name of the Player
    pub name: String,
    /// class of player
    pub class: Option<Class>,
}
impl Player {
    pub fn new(name: impl AsRef<str>, class: Class) -> Self {
        Self {
            name: name.as_ref().into(),
            class: Some(class),
        }
    }
    /// not yet initialized
    /// use in this case at your own risk
    pub fn is_unset(&self) -> bool {
        self == &Self::default()
    }
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_unset() {
            write!(f, "{{waiting for player...}}")?;
            return Ok(());
        }
        write!(
            f,
            "{}{}",
            self.name,
            if let Some(class) = self.class {
                [", ", &class.to_string()].concat()
            } else {
                String::new()
            }
        )
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default, Hash,
)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
/// a class that a player attends in a school, institution
/// format: <grade: number, 0-255><id: any character: Unicode scalar value>
pub struct Class {
    /// the number of years already spent in the institution, whatever. eg: 10
    pub grade: u8,
    /// the id of the class, eg: C
    pub id: char,
}
impl Class {
    pub fn new(grade: u8, id: char) -> Self {
        Self { grade, id }
    }
}
impl TryFrom<&str> for Class {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut x = value.chars();
        let id = x.next_back().ok_or("invalid class id")?;
        let numbers = x.filter(char::is_ascii_digit).collect::<String>();
        let grade = numbers.parse::<u8>().map_err(|_| "invalid grade number")?;
        Ok(Self { grade, id })
    }
}
impl From<Class> for String {
    fn from(value: Class) -> Self {
        format!("{}{}", value.grade, value.id)
    }
}
impl std::fmt::Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = (*self).into();
        write!(f, "{s}")
    }
}

/// A Duel/Match between two players.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Duel {
    pub homie: Player,
    pub guest: Player,
    /// homie won: true, opponent won: false
    pub outcome: Option<bool>,
}
impl std::fmt::Display for Duel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let outcome = self.outcome.map(|oc| {
            if oc {
                (&self.homie, &self.guest)
            } else {
                (&self.guest, &self.homie)
            }
        });
        if let Some((winner, loser)) = outcome {
            write!(f, "winner: {winner} <-> {loser} :loser")
        } else {
            write!(f, "{} <-> {}", self.homie, self.guest)
        }
    }
}
impl Duel {
    pub fn new(homie: Player, guest: Player) -> Self {
        Self {
            homie,
            guest,
            outcome: None,
        }
    }
    /// `self` but with `outcome`
    pub fn with_outcome(self, outcome: Option<bool>) -> Self {
        Self { outcome, ..self }
    }
    #[cfg(test)]
    pub fn get_outcome(self) -> Result<Duel, ()> {
        Ok(self.clone().with_outcome(Some(true)))
    }
    #[cfg(not(test))]
    pub fn get_outcome(self) -> Result<Duel, ()> {
        print!("winner: ");
        std::io::stdout().flush().map_err(|_| ())?;
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).map_err(|_| ())?;
        let outcome = match buf.trim() {
            "<" | "homie" => Some(true),
            ">" | "guest" => Some(false),
            name => {
                let name = name.to_lowercase();
                if self.homie.name.to_lowercase().contains(&name) {
                    Some(true)
                } else if self.guest.name.to_lowercase().contains(&name) {
                    Some(false)
                } else {
                    // dbg!(&name);
                    if matches!(name.as_str(), "q" | "quit" | "exit") {
                        std::process::exit(0);
                    }
                    return Err(());
                }
            }
        };
        // println!("{self}");
        Ok(self.clone().with_outcome(outcome))
    }
    /// take winner of the game
    ///
    /// # Note
    ///
    /// it's taken: moved and replaced by [`Player::default()`]
    ///
    /// # Panics
    ///
    /// if there's no outcome yet
    fn take_winner(&mut self) -> Player {
        if self.outcome.unwrap() {
            std::mem::take(&mut self.homie)
        } else {
            std::mem::take(&mut self.guest)
        }
    }
    /// take loser of the game
    ///
    /// # Note
    ///
    /// it's taken: moved and replaced by [`Player::default()`]
    ///
    /// # Panics
    ///
    /// if there's no outcome yet
    fn take_loser(&mut self) -> Player {
        if self.outcome.unwrap() {
            std::mem::take(&mut self.guest)
        } else {
            std::mem::take(&mut self.homie)
        }
    }

    /// play the [`Duel`]: get an outcome with `read_outcome`
    pub fn play(self) -> (Player, Player) {
        loop {
            if let Ok(mut with_outcome) = self.clone().get_outcome() {
                return (with_outcome.take_winner(), with_outcome.take_loser());
            }
            println!("invalid input");
        }
    }
    /// # Info
    ///
    /// - creates [`Duel`] from first two [`Player`]s of `branch`
    /// - plays the [`Duel`]
    /// - winner get's pushed back to the `branch`
    /// - loser get's returned
    ///
    /// # Warning
    ///
    /// there's a `println!()` hidden in here
    pub fn handle_special(branch: &mut Players) -> Player {
        let (homie, guest) = (branch.0.remove(0), branch.0.swap_remove(0)); // remove first two
        let duel = Duel::new(homie, guest); // create a duel
        println!("{duel}");
        let (winner, loser) = duel.play(); // play it
        branch.0.push(winner); // winner stays
        loser
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_from() {
        let exp = Class::new(0, 'A');
        assert_eq!(Ok(exp), "00A".try_into());
    }
}
