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
            eprintln!("players: {players:?}");
            eprintln!("res: {res:?}");
            let mut idx = 0;
            let cnt = players.remove(0);
            if cnt.class != players[0].class {
                idx = 0;
            } else {
                players.iter().find(|p| {
                    let res = p.class[..2] != cnt.class[..2];
                    idx += 1;
                    res
                });
                idx = 0;
            }

            let pair = (cnt, players.remove(idx));
            res.push(pair);
        }
        res
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
