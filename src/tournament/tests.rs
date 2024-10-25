use super::*;

fn tournament() -> Tournament<backend::Test> {
    let tment = Tournament::new(backend::Test);
    let exp = Tournament {
        winner_branch: vec![],
        loser_branch: vec![],
        knocked: Players::default(),
        _backend: backend::Test,
    };
    assert_eq!(exp, tment);
    tment.players_from_path("data.csv").unwrap()
}

#[test]
fn from_path() {
    let tment = tournament();
    let nu_p = players::tests::nu_p;
    let xp_wb = Players(vec![
        nu_p("Relative Wrasse", 10, 'C'),
        nu_p("Exotic Skunk", 00, 'A'),
        nu_p("Profound Ponytail", 00, 'B'),
        nu_p("Inviting Pheasant", 12, 'B'),
        nu_p("Usable Bengal", 9, 'C'),
        nu_p("Droll Jaguar", 12, 'C'),
        nu_p("Central Mite", 10, 'D'),
        nu_p("Expectant Wolfhound", 9, 'D'),
    ]);
    let xp_lb = Players(vec![nu_p("Casual Ptarmigan", 11, 'B')]);
    let exp = Tournament {
        winner_branch: xp_wb.into_vec_duel(backend::Test::shuffle),
        loser_branch: xp_lb.into_vec_duel(backend::Test::shuffle),
        knocked: Players::default(),
        _backend: backend::Test,
    };
    assert_eq!(exp, tment);
}
