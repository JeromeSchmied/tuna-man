use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub(crate) struct Player {
    /// name of the Player
    pub(crate) name: String,
    /// class of player
    pub(crate) class: Option<Class>,
}
impl Player {
    pub fn new(name: impl AsRef<str>, class: Class) -> Self {
        Self {
            name: name.as_ref().into(),
            class: Some(class),
        }
    }
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
        let class = if let Some(class) = self.class {
            format!(
                ", {}",
                if class.grade == 0 {
                    format!("9Ny{}", class.id)
                } else {
                    class.into()
                }
            )
        } else {
            "".into()
        };
        write!(f, "{}{class}", self.name)
    }
}

#[derive(
    Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
#[serde(try_from = "&str")]
#[serde(into = "String")]
/// two u8s: grade: 00, 09, 10, 11, 12
/// char: id: A,B,C,D for now
pub(crate) struct Class {
    pub(crate) grade: u8,
    pub(crate) id: char,
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
        let Some(id) = x.next_back() else {
            return Err("invalid class id");
        };
        let numbers = x.filter(|x| x.is_ascii_digit()).collect::<String>();
        let Ok(grade) = numbers.parse::<u8>() else {
            return Err("invalid grade number");
        };
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
pub(crate) struct Duel {
    pub(crate) homie: Player,
    pub(crate) guest: Player,
    /// homie won: true, opponent won: false
    pub(crate) outcome: Option<bool>,
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
impl From<(Player, Player)> for Duel {
    fn from(val: (Player, Player)) -> Self {
        Self {
            homie: val.0,
            guest: val.1,
            outcome: None,
        }
    }
}
impl Duel {
    fn winner(&mut self) -> Player {
        if self.outcome.is_some_and(|oc| oc) {
            std::mem::take(&mut self.homie)
        } else {
            std::mem::take(&mut self.guest)
        }
    }
    fn loser(&mut self) -> Player {
        if self.outcome.is_some_and(|oc| oc) {
            std::mem::take(&mut self.guest)
        } else {
            std::mem::take(&mut self.homie)
        }
    }
    fn with_outcome(self, outcome: bool) -> Self {
        Self {
            outcome: Some(outcome),
            ..self
        }
    }
    fn read_outcome(&mut self) -> Result<(), ()> {
        print!("winner: ");
        if std::io::stdout().flush().is_err() {
            return Err(());
        };
        let mut buf = String::new();
        if std::io::stdin().read_line(&mut buf).is_err() {
            return Err(());
        };
        self.outcome = match buf.trim() {
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
        Ok(())
    }
    pub(crate) fn play(mut self) -> (Player, Player) {
        while self.read_outcome().is_err() {
            println!("invalid input");
        }
        (self.winner(), self.loser())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duel_from_players_tuple() {
        let homie = Player::new("Prisca Virtus", Class::new(0, 'D'));
        let guest = Player::new("Prius Quam", Class::new(12, 'B'));
        let duel = Duel {
            homie: homie.clone(),
            guest: guest.clone(),
            outcome: None,
        };
        assert_eq!(duel, (homie, guest).into());
    }
    #[test]
    fn class_from() {
        let exp = Class::new(0, 'A');
        assert_eq!(Ok(exp), "00A".try_into());
    }
}
