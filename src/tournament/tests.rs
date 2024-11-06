use super::*;
use structs::Player;

type BT = backend::Test;
const B: BT = backend::Test;

mod double_elemination {
    use super::*;
    use pretty_assertions::assert_eq;

    type DE = format::DoubleElemination;

    fn tournament() -> Tournament<BT, DE> {
        let tment = Tournament::new(B, format::DoubleElemination::default());
        let exp = Tournament {
            format: format::DoubleElemination::default(),
            _backend: B,
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
        let exp_f = format::DoubleElemination::new(
            xp_wb.into_duels(BT::shuffle),
            xp_lb.into_duels(BT::shuffle),
            Players::default(),
        );
        let exp = Tournament {
            format: exp_f,
            _backend: B,
        };
        assert_eq!(exp, tment);
    }

    #[test]
    fn tment() {
        let mut tment = tournament();
        let nu_p = players::tests::nu_p;
        let test_eq = |xp_bs: (Players, Players, Players), tment: &Tournament<BT, DE>| {
            let exp_f = format::DoubleElemination::new(
                xp_bs.0.into_duels(BT::shuffle),
                xp_bs.1.into_duels(BT::shuffle),
                xp_bs.2,
            );
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
}

mod single_elemination {
    use super::*;
    use pretty_assertions::assert_eq;

    type SE = format::SingleElemination;

    fn tournament() -> Tournament<BT, SE> {
        let tment = Tournament::new(B, format::SingleElemination::default());
        let exp = Tournament {
            format: format::SingleElemination::default(),
            _backend: B,
        };
        assert_eq!(exp, tment);
        tment.players_from_path("data.csv").unwrap()
    }

    #[test]
    fn from_path() {
        let tment = tournament();
        let nu_p = players::tests::nu_p;
        let xp_b = Players(vec![
            nu_p("Relative Wrasse", 10, 'C'),
            nu_p("Exotic Skunk", 00, 'A'),
            nu_p("Profound Ponytail", 00, 'B'),
            nu_p("Inviting Pheasant", 12, 'B'),
            nu_p("Usable Bengal", 9, 'C'),
            nu_p("Droll Jaguar", 12, 'C'),
            nu_p("Central Mite", 10, 'D'),
            nu_p("Expectant Wolfhound", 9, 'D'),
        ]);
        let xp_k = Players(vec![nu_p("Casual Ptarmigan", 11, 'B')]);
        let exp_f = format::SingleElemination::new(xp_b.into_duels(BT::shuffle), xp_k);
        let exp = Tournament {
            format: exp_f,
            _backend: B,
        };
        assert_eq!(exp, tment);
    }

    #[test]
    fn tment() {
        let mut tment = tournament();
        let nu_p = players::tests::nu_p;
        let test_eq = |xp_bs: (Players, Players), tment: &Tournament<BT, SE>| {
            let exp_f = format::SingleElemination::new(xp_bs.0.into_duels(BT::shuffle), xp_bs.1);
            let exp_tm = Tournament {
                format: exp_f,
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
            ),
            gen_bs(
                &[
                    nu_p("Central Mite", 10, 'D'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Relative Wrasse", 10, 'C'),
                    nu_p("Usable Bengal", 9, 'C'),
                ],
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Exotic Skunk", 00, 'A'),
                ],
            ),
            gen_bs(
                &[
                    nu_p("Relative Wrasse", 10, 'C'),
                    nu_p("Central Mite", 10, 'D'),
                ],
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Profound Ponytail", 00, 'B'),
                ],
            ),
            gen_bs(
                &[],
                &[
                    nu_p("Casual Ptarmigan", 11, 'B'),
                    nu_p("Expectant Wolfhound", 9, 'D'),
                    nu_p("Droll Jaguar", 12, 'C'),
                    nu_p("Inviting Pheasant", 12, 'B'),
                    nu_p("Exotic Skunk", 00, 'A'),
                    nu_p("Usable Bengal", 9, 'C'),
                    nu_p("Profound Ponytail", 00, 'B'),
                    nu_p("Central Mite", 10, 'D'),
                    nu_p("Relative Wrasse", 10, 'C'),
                ],
            ),
        ];

        for xp_bs in xp_bs {
            test_eq(xp_bs, &tment);
            tment.play_next_round();
        }
    }
}
