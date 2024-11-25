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
