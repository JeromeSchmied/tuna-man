use std::io::Write;

// use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use serde::{Deserialize, Serialize};

pub mod ui;

const FNAME: &str = "data.csv";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Player {
    /// name of the Player
    name: String,
    /// first two chars: grade: 00, 09, 10, 11, 12
    /// last char: id: A,B,C,D for now
    class: String, // TODO: don't shoot at this little birdie with such a cannon
    points: u8,
}
impl Player {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Players {
    players: Vec<Player>,
}
impl From<Players> for Vec<Match> {
    fn from(players: Players) -> Self {
        if players.players.len() % 2 != 0 {
            panic!("error: number of players should be even");
        }
        players
            .transform()
            .into_iter()
            .map(std::convert::Into::into)
            .collect()
    }
}
impl Players {
    pub fn load() -> Self {
        let mut reader = csv::Reader::from_path(FNAME).unwrap();
        let players = reader.deserialize().flatten().collect();
        Self { players }
    }
    pub fn save(self) {
        let mut writer = csv::Writer::from_path(FNAME).unwrap();
        self.players
            .iter()
            .for_each(|p| writer.serialize(p).unwrap());
        writer.flush().unwrap();
    }
    // pub fn shuffle(&mut self) {
    //     fastrand::shuffle(&mut self.players);
    // }
    pub fn transform(&self) -> Vec<(Player, Player)> {
        let mut players = self.players.clone();
        fastrand::shuffle(&mut players);
        let mut res = Vec::new();
        while !players.is_empty() {
            // dbg!(&players);
            // dbg!(&res);
            let cnt = players.remove(0);
            let idx = Self::diff_list(&players, &cnt)
                .expect("possibly number of players isn't divisible by two");
            let pair = (cnt, players.remove(idx));
            res.push(pair);
        }
        res
    }
    /// # Usage
    ///
    /// returns index
    ///
    /// # Implementation
    ///
    /// calculate diff_list, move the one with highest value from players to results
    /// calculation: least similar class:
    /// 1 same class: grade+id
    /// 2 same id
    /// 3 same grade
    /// 4 same name
    /// 5 nothing in common (based on known things) cool!
    fn diff_list(haystack: &[Player], hay: &Player) -> Option<usize> {
        // dbg!(hay);
        // dbg!(haystack);
        // index, value
        let mut max: (Option<usize>, u8) = (None, 0);
        for (i, p) in haystack.iter().enumerate() {
            let diff = if hay.class == p.class {
                1
            } else if hay.class[..2] == p.class[..2] {
                2
            } else if hay.class.chars().nth(2) == p.class.chars().nth(2) {
                // dbg!(hay);
                // dbg!(p);
                3
            } else if hay.name.split_whitespace().next() == p.name.split_whitespace().next()
                || hay.name.split_whitespace().next_back() == p.name.split_whitespace().next_back()
            {
                4
            } else {
                5
            };
            if diff > max.1 {
                max.1 = diff;
                max.0 = Some(i);
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
        let outcome_disp = if let Some(oc) = self.outcome {
            if oc {
                self.homie.name.as_str()
            } else {
                self.guest.name.as_str()
            }
        } else {
            ""
        };
        write!(
            f,
            "{} <-> {}\t-\t{outcome_disp}",
            self.homie.name, self.guest.name
        )
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
    fn winner(&self) -> Player {
        if self.outcome.is_some_and(|oc| oc) {
            self.homie.clone()
        } else {
            self.guest.clone()
        }
    }
    fn loser(&self) -> Player {
        if self.outcome.is_some_and(|oc| oc) {
            self.guest.clone()
        } else {
            self.homie.clone()
        }
    }
    fn with_outcome(self, outcome: bool) -> Self {
        Self {
            outcome: Some(outcome),
            ..self
        }
    }
    fn read_outcome(&mut self) {
        print!("outcome: ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        self.outcome = match buf.trim() {
            "1" | "homie" => Some(true),
            "0" | "guest" => Some(false),
            name => {
                if self.homie.name.contains(name) {
                    Some(true)
                } else if self.guest.name.contains(name) {
                    Some(false)
                } else {
                    unreachable!("invalid input");
                }
            }
        };
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
    pub fn with_players(self, players: Players) -> Self {
        Self {
            winning: players.into(),
            ..self
        }
    }
    fn play_next_round(&mut self) {
        let mut new_win = Players::default();
        let mut new_lose = Players::default();
        // get outcomes
        for w_match in self.winning.iter_mut() {
            println!("match: {w_match}");
            w_match.read_outcome();
            let mut winner = w_match.winner();
            winner.points += 1;
            new_win.players.push(winner);

            let loser = w_match.loser();
            new_lose.players.push(loser);
        }
        for l_match in self.losing.iter_mut() {
            println!("match: {l_match}");
            l_match.read_outcome();
            let mut winner = l_match.winner();
            winner.points += 1;
            new_lose.players.push(winner);
        }
        // if self.winning.len() == 1 || self.losing.len() == 1 {
        //     let w_match = self.winning[0];
        //     println!("winning: {w_match}");
        //     w_match.read_outcome();
        //     let w_winner = w_match.winner();

        //     let l_match = self.losing[0];
        //     println!("losing: {l_match}");
        //     l_match.read_outcome();
        //     let l_winner =
        //     let last_match = Match {
        //         homie: w_winner,
        //         guest: l_match,
        //         outcome: None,
        //     };
        //     println!("match: {last_match}");
        //     last_match.read_outcome();
        //     new_win.players.push(self.winning[0].clone());
        //     std::process::exit(0);
        // }
        dbg!(&new_win);
        dbg!(&new_lose);
        // TODO: 5th place stuff
        if new_lose.players.len() == 3 {
            new_lose.players.pop();
            // std::process::exit(0);
        }
        if new_win.players.len() == 1 {
            if new_lose.players.len() == 2 {
                let l_match: Vec<Match> = new_lose.clone().into();
                let mut l_match = l_match[0].clone();
                println!("l_match: {l_match}");
                l_match.read_outcome();
                let mut second = l_match.winner();
                second.points += 1;
                println!("winner: {:?}", new_win.players[0]);
                println!("second: {:?}", second);
                std::process::exit(0);
            }
            let l_match: Vec<Match> = new_lose.clone().into();
            let mut l_match = l_match[0].clone();
            println!("l_match: {l_match}");
            l_match.read_outcome();
            let mut winner = l_match.winner();
            winner.points += 1;
            new_win.players.push(winner);
            // std::process::exit(0);
        }
        *self = Self {
            winning: new_win.into(),
            losing: new_lose.into(),
        };
    }
    // pub fn fill_tables(&mut self) {
    //     let wating = if self.players.players.len() % 2 != 0 {
    //         self.players.players.pop()
    //     } else {
    //         None
    //     };
    //     let mut pairs = self.players.transform();
    //     for table in self.tables.iter_mut() {
    //         let pair = if let Some(pp) = pairs.pop() {
    //             (Some(pp.0), Some(pp.1))
    //         } else {
    //             (None, None)
    //         };
    //         table.homie = pair.0;
    //         table.opponent = pair.1;
    //     }
    // }
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
    let players = Players::load();
    // let tables = vec![Table::default(); 4];
    let mut app = App::default().with_players(players);
    println!("{app:#?}");
    while !app.winning.is_empty() && !app.winning.is_empty() {
        app.play_next_round();
        println!("{app:#?}");
    }

    // players.save();

    Ok(())

    // let mut terminal = ratatui::try_init()?;

    // let res = App::default().execute(&mut terminal);

    // ratatui::try_restore()?;

    // res
}
