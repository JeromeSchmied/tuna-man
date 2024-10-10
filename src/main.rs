use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use serde::{Deserialize, Serialize};

pub mod ui;

const FNAME: &str = "data.csv";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Player {
    /// name of the Player
    name: String,
    /// first two chars: grade: 00, 09, 10, 11, 12
    /// last char: id: A,B,C,D for now
    class: String,
    points: u8,
}
impl Player {}

#[derive(Debug, Clone, PartialEq)]
struct Players {
    players: Vec<Player>,
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
    pub fn shuffle(&mut self) {
        fastrand::shuffle(&mut self.players);
    }
    pub fn transform(&self) -> Vec<(Player, Player)> {
        let mut players = self.players.clone();
        let mut res = Vec::new();
        while !players.is_empty() {
            dbg!(&players);
            dbg!(&res);
            let cnt = players.remove(0);
            let idx = Self::diff_list(&self.players, &cnt)
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
    /// TODO: calculate diff_list, move the one with highest value from players to results
    /// calculation: least similar class:
    /// 0. same class: grade+id
    /// 1. same id
    /// 2. same grade
    fn diff_list(haystack: &[Player], hay: &Player) -> Option<usize> {
        // index, value
        let mut max: (Option<usize>, u8) = (None, 0);
        for (i, p) in haystack.iter().enumerate() {
            let diff = if hay.class == p.class {
                1 // same class
            } else if hay.class[..2] == p.class[..2] {
                2 // same class-grade
            } else if hay.class[2..2] == p.class[2..2] {
                3 // same class-id
            } else if hay.name.split_whitespace().next() == p.name.split_whitespace().next()
                || hay.name.split_whitespace().next_back() == p.name.split_whitespace().next_back()
            {
                4 // same name (first or the other)
            } else {
                5 // nothing matches, best one!
            };
            if diff > max.1 {
                max.1 = diff;
                max.0 = Some(i);
            }
        }
        max.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct App {
    num_tables: usize,
    players: Vec<Player>,
}
impl App {
    pub fn new(players: Vec<Player>, num_tables: usize) -> Self {
        Self {
            players,
            num_tables,
        }
    }
    pub fn execute(
        &mut self,
        term: &mut ratatui::Terminal<impl ratatui::backend::Backend>,
    ) -> std::io::Result<()> {
        todo!("app logic");
        loop {
            term.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('j') | KeyCode::Down => todo!(),
                    KeyCode::Char('k') | KeyCode::Up => todo!(),
                    KeyCode::Char('r') => todo!(),
                    KeyCode::Char('n') | KeyCode::Char('l') | KeyCode::Right => todo!(),
                    KeyCode::Char('p') | KeyCode::Char('h') | KeyCode::Left => todo!(),
                    KeyCode::Char('R') | KeyCode::Backspace => todo!(),
                    _ => {}
                }
            } else {
                // resize and restart
            }
        }
        Ok(())
    }
}
fn main() -> std::io::Result<()> {
    let mut players = Players::load();
    players.shuffle();
    let pairs = players.transform();
    println!("pairs: {pairs:#?}");

    // players.save();

    return Ok(());

    let mut terminal = ratatui::try_init()?;

    let res = App::default().execute(&mut terminal);

    ratatui::try_restore()?;

    res
}
