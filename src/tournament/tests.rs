use super::*;
use structs::Player;

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
        winner_branch: xp_wb.into_duels(backend::Test::shuffle),
        loser_branch: xp_lb.into_duels(backend::Test::shuffle),
        knocked: Players::default(),
        _backend: backend::Test,
    };
    assert_eq!(exp, tment);
}

#[test]
fn tment() {
    let mut tment = tournament();
    let nu_p = players::tests::nu_p;
    let test_eq = |xp_bs: (Players, Players, Players), tment: &Tournament<backend::Test>| {
        let tm = Tournament {
            winner_branch: xp_bs.0.into_duels(backend::Test::shuffle),
            loser_branch: xp_bs.1.into_duels(backend::Test::shuffle),
            knocked: xp_bs.2,
            _backend: backend::Test,
        };
        assert_eq!(&tm, tment);
    };

    let gen_bs = |wb: &[Player], lb: &[Player], kb: &[Player]| -> (Players, Players, Players) {
        (Players(wb.into()), Players(lb.into()), Players(kb.into()))
    };
    // eXPected BrancheS
    let xp_bs = vec![
        gen_bs(
            &[
                nu_p("Relative Wrasse", 10, 'C'),
                nu_p("Exotic Skunk", 00, 'A'),
                nu_p("Profound Ponytail", 00, 'B'),
                nu_p("Inviting Pheasant", 12, 'B'),
                nu_p("Usable Bengal", 9, 'C'),
                nu_p("Droll Jaguar", 12, 'C'),
                nu_p("Central Mite", 10, 'D'),
                nu_p("Expectant Wolfhound", 9, 'D'),
            ],
            &[nu_p("Casual Ptarmigan", 11, 'B')],
            &[],
        ),
        gen_bs(
            &[
                nu_p("Central Mite", 10, 'D'),
                nu_p("Profound Ponytail", 00, 'B'),
                nu_p("Relative Wrasse", 10, 'C'),
                nu_p("Usable Bengal", 9, 'C'),
            ],
            &[
                nu_p("Droll Jaguar", 12, 'C'),
                nu_p("Exotic Skunk", 00, 'A'),
                nu_p("Inviting Pheasant", 12, 'B'),
                nu_p("Expectant Wolfhound", 9, 'D'),
            ],
            &[nu_p("Casual Ptarmigan", 11, 'B')],
        ),
        gen_bs(
            &[
                nu_p("Relative Wrasse", 10, 'C'),
                nu_p("Central Mite", 10, 'D'),
            ],
            &[
                nu_p("Usable Bengal", 9, 'C'),
                nu_p("Inviting Pheasant", 12, 'B'),
                nu_p("Droll Jaguar", 12, 'C'),
                nu_p("Profound Ponytail", 00, 'B'),
            ],
            &[
                nu_p("Casual Ptarmigan", 11, 'B'),
                nu_p("Expectant Wolfhound", 9, 'D'),
                nu_p("Exotic Skunk", 00, 'A'),
            ],
        ),
        gen_bs(
            &[nu_p("Relative Wrasse", 10, 'C')],
            &[nu_p("Droll Jaguar", 12, 'C'), nu_p("Central Mite", 10, 'D')],
            &[
                nu_p("Casual Ptarmigan", 11, 'B'),
                nu_p("Expectant Wolfhound", 9, 'D'),
                nu_p("Exotic Skunk", 00, 'A'),
                nu_p("Profound Ponytail", 00, 'B'),
                nu_p("Inviting Pheasant", 12, 'B'),
                nu_p("Usable Bengal", 9, 'C'),
            ],
        ),
        gen_bs(
            &[],
            &[],
            &[
                nu_p("Casual Ptarmigan", 11, 'B'),
                nu_p("Expectant Wolfhound", 9, 'D'),
                nu_p("Exotic Skunk", 00, 'A'),
                nu_p("Profound Ponytail", 00, 'B'),
                nu_p("Inviting Pheasant", 12, 'B'),
                nu_p("Usable Bengal", 9, 'C'),
                nu_p("Central Mite", 10, 'D'),
                nu_p("Droll Jaguar", 12, 'C'),
                nu_p("Relative Wrasse", 10, 'C'),
            ],
        ),
    ];

    for xp_bs in xp_bs {
        test_eq(xp_bs, &tment);
        tment.play_next_round();
    }
}
