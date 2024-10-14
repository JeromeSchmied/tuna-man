use std::io::Write;

// use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use serde::{Deserialize, Serialize};

pub mod ui;

// TODO: pass as arg, maybe use [`clap`](https://lib.rs/crates/clap)
const FNAME: &str = "data.csv";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
struct Player {
    /// name of the Player
    name: String,
    /// first two chars: grade: 00, 09, 10, 11, 12
    /// last char: id: A,B,C,D for now
    class: String, // TODO: don't shoot at this little birdie with such a cannon if possible
}
impl Player {
    fn grade(&self) -> &str {
        &self.class[..2]
    }
    fn class_id(&self) -> char {
        self.class.chars().next_back().unwrap()
    }
}
impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class = if self.grade() == "00" {
            "9Ny"
        } else {
            self.grade()
        };
        let class = format!("{class}{}", self.class.chars().next_back().unwrap());
        write!(f, "{} - {class}", self.name)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Players(Vec<Player>);
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
impl Players {
    fn load() -> std::io::Result<Self> {
        let mut reader = csv::Reader::from_path(FNAME)?;
        let players = reader.deserialize().flatten().collect();
        Ok(Self(players))
    }
    fn save(self) -> std::io::Result<()> {
        let mut writer = csv::Writer::from_path(FNAME)?;
        self.0.iter().for_each(|p| writer.serialize(p).unwrap());
        writer.flush()
    }
    fn sort_as_pairs(&mut self) {
        if self.0.is_empty() {
            return;
        }
        fastrand::shuffle(&mut self.0);
        let mut as_pairs = Vec::new();
        while self.0.len() > 1 {
            let cnt = self.0.remove(0);
            let idx = Self::diff_list(&self.0, &cnt)
                .expect("possibly number of players isn't divisible by two");
            as_pairs.push(cnt);
            as_pairs.push(self.0.remove(idx));
        }
        if self.0.len() == 1 {
            as_pairs.push(self.0.pop().unwrap());
        }
        self.0 = as_pairs;
    }
    fn transform(&self) -> Vec<(Player, Player)> {
        let mut players = self.clone();
        assert_eq!(players.0.len() % 2, 0);
        players.sort_as_pairs();
        let mut res = Vec::new();
        while !players.0.is_empty() {
            let cnt = players.0.remove(0);
            let next = players.0.remove(0);
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
        // dbg!(hay);
        // dbg!(haystack);
        // index, value
        let mut max: (Option<usize>, u8) = (None, 0);
        for (i, p) in haystack.iter().enumerate() {
            let diff = if hay.class == p.class {
                // same class
                1
            } else if hay.grade() == p.grade() {
                // same grade
                2
            } else if hay.class_id() == p.class_id() {
                // same class id
                3
            } else {
                4
            };
            if diff > max.1 {
                max.1 = diff;
                max.0 = Some(i);
                // found one that's already highest value, use it
                if max.0 == Some(4) {
                    break;
                }
            }
        }
        // dbg!(max);
        max.0
    }
}

// #[derive(Clone, Debug, PartialEq, Eq, Default)]
// struct Table {
//     homie: Option<Player>,
//     opponent: Option<Player>,
// }
#[derive(Clone, Debug, PartialEq, Eq)]
struct Match {
    homie: Player,
    guest: Player,
    /// homie won: true, opponent won: false
    outcome: Option<bool>,
}
impl std::fmt::Display for Match {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <-> {}", self.homie.name, self.guest.name)
    }
}
impl From<(Player, Player)> for Match {
    fn from(val: (Player, Player)) -> Self {
        Self {
            homie: val.0,
            guest: val.1,
            outcome: None,
        }
    }
}
impl Match {
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
    fn read_outcome(&mut self) {
        print!("winner: ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        self.outcome = match buf.trim() {
            "<" | "homie" => Some(true),
            ">" | "guest" => Some(false),
            name => {
                let name = name.to_ascii_lowercase();
                if self.homie.name.contains(&name) {
                    Some(true)
                } else if self.guest.name.contains(&name) {
                    Some(false)
                } else {
                    unreachable!("invalid input");
                }
            }
        };
    }
    fn play(mut self) -> (Player, Player) {
        self.read_outcome();
        (self.winner(), self.loser())
    }
}

impl From<Players> for App {
    fn from(players: Players) -> Self {
        let mut new_win = players;
        let mut new_lose = Players::default();
        if new_win.0.len() % 2 == 1 {
            new_win.sort_as_pairs(); // shuffle
            let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_match = Match {
                homie,
                guest,
                outcome: None,
            }; // create a match
            println!("\nspecial winner match: {w_match}");
            let (winner, loser) = w_match.play(); // play it
            new_win.0.push(winner); // winner stays
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }
        Self {
            winning: new_win.into(),
            losing: new_lose.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct App {
    winning: Vec<Match>,
    losing: Vec<Match>,
}
impl App {
    // pub fn with_tables(self, tables: &[Table]) -> Self {
    //     Self {
    //         tables: tables.into(),
    //         ..self
    //     }
    // }
    fn play_next_round(&mut self) {
        let mut new_win = Players::default();
        let mut new_lose = Players::default();
        // get outcomes
        while let Some(w_match) = self.winning.pop() {
            if w_match.guest == Player::default() {
                new_win.0.push(w_match.homie);
                break;
            }
            println!("\nwinner match: {w_match}");
            let (winner, loser) = w_match.play();
            new_win.0.push(winner);
            new_lose.0.push(loser);
        }
        println!("-----------------");
        while let Some(l_match) = self.losing.pop() {
            if l_match.guest == Player::default() {
                new_lose.0.push(l_match.homie);
                break;
            }
            println!("\nloser match: {l_match}");
            let (winner, loser) = l_match.play();
            new_lose.0.push(winner);
            println!("bye-bye {loser}");
        }

        if new_win.0.len() == 1 {
            println!(
                "probably should end soon, only winner branch remainder: {}",
                new_win.0[0]
            );
        } else if new_win.0.len() % 2 == 1 {
            new_win.sort_as_pairs(); // shuffle
            let (homie, guest) = (new_win.0.swap_remove(0), new_win.0.swap_remove(0)); // remove first two
            let w_match = Match {
                homie,
                guest,
                outcome: None,
            }; // create a match
            println!("\nspecial winner match: {w_match}");
            let (winner, loser) = w_match.play(); // play it
            new_win.0.push(winner); // winner stays
            new_lose.0.push(loser); // loser get's pushed to loser branch
        }

        if new_lose.0.len() == 1 {
            let homie = new_win.0.pop().unwrap();
            let guest = new_lose.0.pop().unwrap();
            let finals = Match {
                homie,
                guest,
                outcome: None,
            };
            println!("FINAL GAME: {finals}");
            let (winner, second) = finals.play();
            println!("WINNER: {winner}");
            println!("SECOND PLACE: {second}");
            return;
        } else if new_lose.0.len() % 2 == 1 {
            new_lose.sort_as_pairs(); // shuffle
            let (homie, guest) = (new_lose.0.swap_remove(0), new_lose.0.swap_remove(0)); // remove first two
            let l_match = Match {
                homie,
                guest,
                outcome: None,
            }; // create a match
            println!("\nspecial loser match: {l_match}");
            let (winner, loser) = l_match.play(); // play it
            new_lose.0.push(winner); // winner stays
            println!("bye-bye {loser}"); // loser get's eleminated
        }
        dbg!(&new_win);
        dbg!(&new_lose);
        *self = Self {
            winning: new_win.into(),
            losing: new_lose.into(),
        };
    }
    // pub fn execute(
    //     &mut self,
    //     term: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
    // ) -> std::io::Result<()> {
    //     todo!("app logic");
    //     loop {
    //         term.draw(|f| self.ui(f))?;

    //         if let Event::Key(key) = event::read()? {
    //             if key.kind != KeyEventKind::Press {
    //                 continue;
    //             }
    //             match key.code {
    //                 KeyCode::Char('q') | KeyCode::Esc => break,
    //                 KeyCode::Char('j') | KeyCode::Down => todo!(),
    //                 KeyCode::Char('k') | KeyCode::Up => todo!(),
    //                 KeyCode::Char('r') => todo!(),
    //                 KeyCode::Char('n') | KeyCode::Char('l') | KeyCode::Right => todo!(),
    //                 KeyCode::Char('p') | KeyCode::Char('h') | KeyCode::Left => todo!(),
    //                 KeyCode::Char('R') | KeyCode::Backspace => todo!(),
    //                 _ => {}
    //             }
    //         } else {
    //             // resize and restart
    //         }
    //     }
    //     Ok(())
    // }
}
fn main() -> std::io::Result<()> {
    let players = Players::load()?;
    // let tables = vec![Table::default(); 4];
    let mut app = App::from(players);
    println!("{app:#?}");
    let mut i = 0;
    while !app.winning.is_empty() || !app.losing.is_empty() {
        app.play_next_round();
        println!("{i}\n{app:#?}");
        i += 1;
    }

    // players.save();

    Ok(())

    // let mut terminal = ratatui::try_init()?;

    // let res = App::default().execute(&mut terminal);

    // ratatui::try_restore()?;

    // res
}
#[test]
fn does_it_contain() {
    let hay = "";
    let haystack = ["One Two", "Three Four", "Plum Pear"];
    assert!(haystack.iter().any(|s| s.contains(hay))); // NOTE: wow! it does.
}
