use super::*;
use players::tests::nu_p;
use structs::Player;

type BT = backend::Test;
const B: BT = backend::Test;

mod double_elemination {
    use super::*;
    use pretty_assertions::assert_eq;

    type DE = format::DoubleElemination;

    fn tournament() -> Tournament<BT, DE> {
        let tment = Tournament::new(B, DE::default());
        let exp = Tournament {
            format: DE::default(),
            _backend: B,
        };
        assert_eq!(exp, tment);
        tment.players_from_path("data.csv").unwrap()
    }

    #[test]
    fn from_path() {
        let tment = tournament();
        let xp_wb = Players(vec![
            nu_p("Casual Ptarmigan", 11, 'B'),
            nu_p("Exotic Skunk", 00, 'A'),
            nu_p("Droll Jaguar", 12, 'C'),
            nu_p("Usable Bengal", 9, 'C'),
            nu_p("Inviting Pheasant", 12, 'B'),
            nu_p("Profound Ponytail", 00, 'B'),
            nu_p("Expectant Wolfhound", 9, 'D'),
            nu_p("Central Mite", 10, 'D'),
        ]);
        let xp_lb = Players(vec![nu_p("Relative Wrasse", 10, 'C')]);
        let exp_f = DE::new(xp_wb, xp_lb, Players::default());
        let exp = Tournament {
            format: exp_f,
            _backend: B,
        };
        assert_eq!(exp, tment);
    }

    #[test]
    fn tment() {
        let mut tment = tournament();
        let test_eq = |xp_bs: (Players, Players, Players), tment: &Tournament<BT, DE>| {
            let exp_f = DE::new(xp_bs.0, xp_bs.1, xp_bs.2);
            let tm = Tournament {
                format: exp_f,
                _backend: B,
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
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Central Mite", 10, 'D'),
                ],
                &[nu_p("Relative Wrasse", 10, 'C')],
                &[],
            ),
            gen_bs(
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Droll Jaguar", 12, 'C'),
                ],
                &[
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Relative Wrasse", 10, 'C'),
                ],
                &[nu_p("Central Mite", 10, 'D')],
            ),
        ];

        for xp_bs in xp_bs {
            test_eq(xp_bs, &tment);
            tment.play_next_round(true);
        }
        assert!(tment.is_end());
    }
}

mod single_elemination {
    use super::*;
    use pretty_assertions::assert_eq;

    type SE = format::SingleElemination;

    #[test]
    fn tment_clean() {
        let team = |name: &&str| Player {
            name: name.to_string(),
            class: None,
        };
        let teams = |teams: &[&str]| teams.iter().map(team).collect::<Vec<_>>();
        let mut tment = Tournament::new(B, SE::default())
            .players_from_path("football-teams.csv")
            .unwrap();

        let test_eq = |xp_bs: (Players, Players), tment: &Tournament<BT, SE>| {
            let exp_tm = Tournament {
                format: SE::new(xp_bs.0, xp_bs.1),
                _backend: B,
            };
            assert_eq!(&exp_tm, tment);
        };
        let gen_bs = |wb: &[&str], kb: &[&str]| (Players(teams(wb)), Players(teams(kb)));
        let xp_bs = [
            gen_bs(
                &[
                    "Germany", "Paraguay", "Mexico", "Usa", "Spain", "Ireland", "Korea", "Italy",
                    "Denmark", "England", "Brazil", "Belgium", "Sweden", "Senegal", "Japan",
                    "Turkey",
                ],
                &[],
            ),
            gen_bs(
                &[
                    "Germany", "Mexico", "Spain", "Korea", "Denmark", "Brazil", "Sweden", "Japan",
                ],
                &[
                    "Paraguay", "Usa", "Ireland", "Italy", "England", "Belgium", "Senegal",
                    "Turkey",
                ],
            ),
            gen_bs(
                &["Germany", "Spain", "Denmark", "Sweden"],
                &[
                    "Paraguay", "Usa", "Ireland", "Italy", "England", "Belgium", "Senegal",
                    "Turkey", "Mexico", "Korea", "Brazil", "Japan",
                ],
            ),
            gen_bs(
                &["Germany", "Denmark"],
                &[
                    "Paraguay", "Usa", "Ireland", "Italy", "England", "Belgium", "Senegal",
                    "Turkey", "Mexico", "Korea", "Brazil", "Japan", "Spain", "Sweden",
                ],
            ),
            gen_bs(
                &[],
                &[
                    "Paraguay", "Usa", "Ireland", "Italy", "England", "Belgium", "Senegal",
                    "Turkey", "Mexico", "Korea", "Brazil", "Japan", "Spain", "Sweden", "Denmark",
                    "Germany",
                ],
            ),
        ];

        for xp_bs in xp_bs {
            test_eq(xp_bs, &tment);
            tment.play_next_round(true);
        }
        assert!(tment.is_end());
    }

    #[test]
    fn tment() {
        let mut tment = {
            let tment = Tournament::new(B, SE::default());
            let exp = Tournament {
                format: SE::default(),
                _backend: B,
            };
            assert_eq!(exp, tment);
            tment.players_from_path("data.csv").unwrap()
        };
        let test_eq = |xp_bs: (Players, Players), tment: &Tournament<BT, SE>| {
            let exp_tm = Tournament {
                format: SE::new(xp_bs.0, xp_bs.1),
                _backend: B,
            };
            assert_eq!(&exp_tm, tment);
        };

        let gen_bs = |wb: &[Player], kb: &[Player]| -> (Players, Players) {
            (Players(wb.into()), Players(kb.into()))
        };
        // eXPected BrancheS
        let xp_bs = vec![
            gen_bs(
                &[
                    nu_p("Central Mite", 10, 'D'),
                    nu_p("Relative Wrasse", 10, 'C'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Casual Ptarmigan", 11, 'B'),
                ],
                &[],
            ),
            gen_bs(
                &[
                    nu_p("Relative Wrasse", 10, 'C'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Central Mite", 10, 'D'),
                ],
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                ],
            ),
            gen_bs(
                &[
                    nu_p("Relative Wrasse", 10, 'C'),
                    nu_p("Central Mite", 10, 'D'),
                ],
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Droll Jaguar", 12, 'C'),
                ],
            ),
            gen_bs(
                &[],
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Central Mite", 10, 'D'),
                    nu_p("Relative Wrasse", 10, 'C'),
                ],
            ),
        ];

        for xp_bs in xp_bs {
            test_eq(xp_bs, &tment);
            tment.play_next_round(true);
        }
        assert!(tment.is_end());
    }
}

mod round_robin {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    use structs::Duel;

    type RR = format::RoundRobin;

    fn tournament() -> Tournament<BT, RR> {
        let tment = Tournament::new(B, RR::default());
        let exp = Tournament {
            format: RR::default(),
            _backend: B,
        };
        assert_eq!(exp, tment);
        tment.players_from_path("data.csv").unwrap()
    }

    #[test]
    fn from_path() {
        let tment = tournament();
        let players = Players(vec![
            nu_p("Central Mite", 10, 'D'),
            nu_p("Relative Wrasse", 10, 'C'),
            nu_p("Exotic Skunk", 00, 'A'),
            nu_p("Droll Jaguar", 12, 'C'),
            nu_p("Usable Bengal", 9, 'C'),
            nu_p("Inviting Pheasant", 12, 'B'),
            nu_p("Profound Ponytail", 00, 'B'),
            nu_p("Expectant Wolfhound", 9, 'D'),
            nu_p("Casual Ptarmigan", 11, 'B'),
            Player::default(),
        ]);
        let duel =
            |i: usize, j: usize| -> Duel { Duel::new(players.0[i].clone(), players.0[j].clone()) };
        let duels = vec![duel(0, 9), duel(1, 8), duel(2, 7), duel(3, 6), duel(4, 5)];
        let points = players.0.iter().cloned().map(|p| (p, 0)).collect();

        let exp_f = RR::new(duels, players, points, 0);

        let exp = Tournament {
            format: exp_f,
            _backend: B,
        };
        assert_eq!(exp, tment);
    }

    #[test]
    fn tment() {
        let mut tment = tournament();

        let players = Players(vec![
            nu_p("Central Mite", 10, 'D'),
            nu_p("Relative Wrasse", 10, 'C'),
            nu_p("Exotic Skunk", 00, 'A'),
            nu_p("Droll Jaguar", 12, 'C'),
            nu_p("Usable Bengal", 9, 'C'),
            nu_p("Inviting Pheasant", 12, 'B'),
            nu_p("Profound Ponytail", 00, 'B'),
            nu_p("Expectant Wolfhound", 9, 'D'),
            nu_p("Casual Ptarmigan", 11, 'B'),
            Player::default(),
        ]);

        let player = |i: usize| players.0[i].clone();
        let duel = |i: usize, j: usize| Duel::new(player(i), player(j));

        let test_eq = |xp: (Vec<Duel>, HashMap<Player, u8>, usize), tment: &Tournament<BT, RR>| {
            let exp_f = RR::new(xp.0, players.clone(), xp.1.clone(), xp.2);

            let xp_tm = Tournament {
                format: exp_f,
                _backend: B,
            };

            assert_eq!(&xp_tm, tment);
        };

        let gen_xp = |duels: &[Duel],
                      points: HashMap<Player, u8>,
                      round: usize|
         -> (Vec<Duel>, HashMap<Player, u8>, usize) {
            (duels.into(), points, round)
        };
        let points = |points: &[u8]| -> HashMap<Player, u8> {
            points
                .iter()
                .enumerate()
                .map(|(i, p)| (player(i), *p))
                .collect()
        };
        // eXPected thingS
        let xps = vec![
            gen_xp(
                &[duel(0, 9), duel(1, 8), duel(2, 7), duel(3, 6), duel(4, 5)],
                points(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
                0,
            ),
            gen_xp(
                &[duel(0, 9), duel(1, 8), duel(2, 7), duel(3, 6), duel(4, 5)],
                points(&[0, 1, 1, 1, 1, 0, 0, 0, 0, 0]),
                1,
            ),
            gen_xp(
                &[duel(0, 8), duel(9, 7), duel(1, 6), duel(2, 5), duel(3, 4)],
                points(&[0, 2, 2, 2, 2, 0, 0, 0, 0, 0]),
                2,
            ),
            gen_xp(
                &[duel(0, 7), duel(8, 6), duel(9, 5), duel(1, 4), duel(2, 3)],
                points(&[1, 3, 3, 3, 2, 0, 0, 0, 0, 0]),
                3,
            ),
            gen_xp(
                &[duel(0, 6), duel(7, 5), duel(8, 4), duel(9, 3), duel(1, 2)],
                points(&[2, 4, 4, 3, 2, 0, 0, 0, 1, 0]),
                4,
            ),
            gen_xp(
                &[duel(0, 5), duel(6, 4), duel(7, 3), duel(8, 2), duel(9, 1)],
                points(&[3, 5, 4, 3, 2, 0, 0, 1, 2, 0]),
                5,
            ),
            gen_xp(
                &[duel(0, 4), duel(5, 3), duel(6, 2), duel(7, 1), duel(8, 9)],
                points(&[4, 5, 4, 3, 2, 0, 1, 2, 3, 0]),
                6,
            ),
            gen_xp(
                &[duel(0, 3), duel(4, 2), duel(5, 1), duel(6, 9), duel(7, 8)],
                points(&[5, 5, 4, 3, 2, 1, 2, 3, 3, 0]),
                7,
            ),
            gen_xp(
                &[duel(0, 2), duel(3, 1), duel(4, 9), duel(5, 8), duel(6, 7)],
                points(&[6, 5, 4, 3, 3, 2, 2, 4, 3, 0]),
                8,
            ),
            gen_xp(
                &[duel(0, 1), duel(2, 9), duel(3, 8), duel(4, 7), duel(5, 6)],
                points(&[7, 5, 4, 4, 3, 3, 3, 4, 3, 0]),
                9,
            ),
        ];
        let mut xps = xps.into_iter();

        while !tment.is_end() {
            let xp = xps.next().unwrap();
            test_eq(xp, &tment);
            tment.play_next_round(false);
        }
    }
}
