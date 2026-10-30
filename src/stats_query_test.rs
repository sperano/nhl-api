use super::*;

fn pairs(query: &StatsQuery) -> Vec<(&'static str, String)> {
    query.query_pairs()
}

fn pair<'a>(pairs: &'a [(&'static str, String)], name: &str) -> Option<&'a str> {
    pairs
        .iter()
        .find(|(key, _)| *key == name)
        .map(|(_, value)| value.as_str())
}

// ===== Filter helpers =====

#[test]
fn test_stats_query_season_helpers_render() {
    let season = StatsQuery::new().season(Season::new(2024));
    assert_eq!(season.cayenne_exp(), "seasonId=20242025");

    let range = StatsQuery::new().seasons(Season::new(2020), Season::new(2023));
    assert_eq!(
        range.cayenne_exp(),
        "seasonId>=20202021 and seasonId<=20232024"
    );
}

#[test]
fn test_stats_query_id_helpers_render() {
    let cases = [
        (
            StatsQuery::new().game_type(GameType::Playoffs),
            "gameTypeId=3",
        ),
        (
            StatsQuery::new().franchise(FranchiseId::new(5)),
            "franchiseId=5",
        ),
        (
            StatsQuery::new().opponent_franchise(FranchiseId::new(6)),
            "opponentFranchiseId=6",
        ),
        (StatsQuery::new().team(TeamId::new(8)), "teamId=8"),
        (
            StatsQuery::new().player(PlayerId::new(8478402)),
            "playerId=8478402",
        ),
        (
            StatsQuery::new().game(GameId::new(2024020001)),
            "gameId=2024020001",
        ),
    ];
    for (query, expected) in cases {
        assert_eq!(query.cayenne_exp(), expected);
    }
}

#[test]
fn test_stats_query_position_uses_stats_api_codes() {
    let cases = [
        (Position::Center, "positionCode='C'"),
        (Position::LeftWing, "positionCode='L'"),
        (Position::RightWing, "positionCode='R'"),
        (Position::Defense, "positionCode='D'"),
        (Position::Goalie, "positionCode='G'"),
    ];
    for (position, expected) in cases {
        assert_eq!(StatsQuery::new().position(position).cayenne_exp(), expected);
    }
}

#[test]
fn test_stats_query_position_forward_matches_all_forward_codes() {
    let query = StatsQuery::new()
        .position(Position::Forward)
        .game_type(GameType::RegularSeason);

    assert_eq!(
        query.cayenne_exp(),
        "(positionCode='C' or positionCode='L' or positionCode='R') and gameTypeId=2"
    );
}

#[test]
fn test_stats_query_player_attribute_helpers_render() {
    let cases = [
        (
            StatsQuery::new().nationality("SWE"),
            "nationalityCode='SWE'",
        ),
        (
            StatsQuery::new().birth_country("USA".to_string()),
            "birthCountryCode='USA'",
        ),
        (StatsQuery::new().draft_year(2015), "draftYear=2015"),
        (StatsQuery::new().draft_round(1), "draftRound=1"),
        (StatsQuery::new().rookie(true), "isRookie=1"),
        (StatsQuery::new().rookie(false), "isRookie=0"),
        (StatsQuery::new().active(), "active=1"),
        (StatsQuery::new().hall_of_fame(), "isInHallOfFame=1"),
        (
            StatsQuery::new().shoots_catches(Handedness::Left),
            "shootsCatches='L'",
        ),
        (StatsQuery::new().home_road(HomeRoad::Home), "homeRoad='H'"),
        (
            StatsQuery::new().decision(GoalieDecision::Win),
            "decision='W'",
        ),
        (
            StatsQuery::new().decision(GoalieDecision::OvertimeLoss),
            "decision='O'",
        ),
    ];
    for (query, expected) in cases {
        assert_eq!(query.cayenne_exp(), expected);
    }
}

#[test]
fn test_stats_query_nationality_escapes_quotes() {
    assert_eq!(
        StatsQuery::new().nationality("A'B").cayenne_exp(),
        r"nationalityCode='A\'B'"
    );
}

#[test]
fn test_stats_query_filters_are_anded_with_or_parenthesized() {
    let query = StatsQuery::new()
        .filter(
            CayenneExpr::field("a")
                .eq(1)
                .or(CayenneExpr::field("b").eq(2)),
        )
        .filter(CayenneExpr::field("c").eq(3));

    assert_eq!(query.cayenne_exp(), "(a=1 or b=2) and c=3");
}

#[test]
fn test_stats_query_without_filters_has_empty_expression() {
    assert_eq!(StatsQuery::new().cayenne_exp(), "");
    assert_eq!(StatsQuery::new().fact_cayenne_exp(), None);
}

#[test]
fn test_stats_query_fact_filters_are_anded() {
    let query = StatsQuery::new()
        .fact_filter(CayenneExpr::field("gamesPlayed").ge(80))
        .fact_filter(CayenneExpr::field("points").gt(100));

    assert_eq!(
        query.fact_cayenne_exp().as_deref(),
        Some("gamesPlayed>=80 and points>100")
    );
    assert_eq!(query.cayenne_exp(), "");
}

// ===== Sorting and paging =====

#[test]
fn test_stats_query_sort_keys_keep_order() {
    let query = StatsQuery::new()
        .sort_by("points", SortDirection::Desc)
        .sort_by("lastName", SortDirection::AscIgnoreCase);

    assert_eq!(
        query.sort_keys(),
        [
            SortKey::new("points", SortDirection::Desc),
            SortKey::new("lastName", SortDirection::AscIgnoreCase),
        ]
    );
}

#[test]
fn test_sort_direction_serializes_to_api_names() {
    let cases = [
        (SortDirection::Asc, "\"ASC\""),
        (SortDirection::Desc, "\"DESC\""),
        (SortDirection::AscIgnoreCase, "\"ASC_CI\""),
        (SortDirection::DescIgnoreCase, "\"DESC_CI\""),
    ];
    for (direction, expected) in cases {
        assert_eq!(serde_json::to_string(&direction).unwrap(), expected);
    }
}

#[test]
fn test_stats_query_mode_accessors() {
    let query = StatsQuery::new();
    assert!(!query.is_aggregate());
    assert!(!query.is_by_game());

    let query = query.aggregate(true).by_game(true);
    assert!(query.is_aggregate());
    assert!(query.is_by_game());
}

// ===== query_pairs =====

#[test]
fn test_stats_query_default_pairs_send_empty_cayenne_exp_and_no_limit() {
    let pairs = pairs(&StatsQuery::new());

    assert_eq!(
        pairs,
        [
            ("cayenneExp", String::new()),
            ("isAggregate", "false".to_string()),
            ("isGame", "false".to_string()),
            ("start", "0".to_string()),
        ]
    );
}

#[test]
fn test_stats_query_pairs_include_every_option() {
    let query = StatsQuery::new()
        .season(Season::new(2024))
        .fact_filter(CayenneExpr::field("gamesPlayed").ge(80))
        .sort_by("points", SortDirection::Desc)
        .sort_by("playerId", SortDirection::Asc)
        .start(100)
        .limit(25)
        .aggregate(true)
        .by_game(true);

    let pairs = pairs(&query);

    assert_eq!(pair(&pairs, "cayenneExp"), Some("seasonId=20242025"));
    assert_eq!(pair(&pairs, "factCayenneExp"), Some("gamesPlayed>=80"));
    assert_eq!(pair(&pairs, "isAggregate"), Some("true"));
    assert_eq!(pair(&pairs, "isGame"), Some("true"));
    assert_eq!(pair(&pairs, "start"), Some("100"));
    assert_eq!(pair(&pairs, "limit"), Some("25"));
    assert_eq!(
        pair(&pairs, "sort"),
        Some(
            r#"[{"property":"points","direction":"DESC"},{"property":"playerId","direction":"ASC"}]"#
        )
    );
}

#[test]
fn test_stats_query_unlimited_sends_limit_minus_one() {
    let pairs = pairs(&StatsQuery::new().unlimited());
    assert_eq!(pair(&pairs, "limit"), Some("-1"));
}

#[test]
fn test_stats_query_last_limit_call_wins() {
    let pairs = pairs(&StatsQuery::new().unlimited().limit(5));
    assert_eq!(pair(&pairs, "limit"), Some("5"));
}

#[test]
fn test_stats_query_omits_unset_optional_pairs() {
    let pairs = pairs(&StatsQuery::new().season(Season::new(2024)));
    assert_eq!(pair(&pairs, "limit"), None);
    assert_eq!(pair(&pairs, "factCayenneExp"), None);
    assert_eq!(pair(&pairs, "sort"), None);
}

// ===== validate =====

#[test]
fn test_stats_query_validate_accepts_typical_query() {
    let query = StatsQuery::new()
        .season(Season::new(2024))
        .position(Position::Forward)
        .nationality("SWE")
        .fact_filter(CayenneExpr::field("gamesPlayed").ge(80))
        .sort_by("points", SortDirection::Desc)
        .limit(10);

    assert_eq!(query.validate(), Ok(()));
    assert_eq!(StatsQuery::new().unlimited().validate(), Ok(()));
}

#[test]
fn test_stats_query_validate_rejects_bad_filter_fields() {
    for bad in ["", "1abc", "a b", "a=1 or b", "a-b", "é", "a.b", "a'"] {
        let query = StatsQuery::new().filter(CayenneExpr::field(bad).eq(1));
        assert_eq!(
            query.validate(),
            Err(StatsQueryError::InvalidField(bad.to_string())),
            "filter field {bad:?}"
        );
    }
}

#[test]
fn test_stats_query_validate_accepts_identifier_fields() {
    for good in ["a", "A", "gamesPlayed", "ev_goals", "a1", "X_9"] {
        let query = StatsQuery::new().filter(CayenneExpr::field(good).eq(1));
        assert_eq!(query.validate(), Ok(()), "filter field {good:?}");
    }
}

#[test]
fn test_stats_query_validate_rejects_bad_fact_filter_field() {
    let query = StatsQuery::new().fact_filter(CayenneExpr::field("x y").eq(1));
    assert_eq!(
        query.validate(),
        Err(StatsQueryError::InvalidField("x y".to_string()))
    );
}

#[test]
fn test_stats_query_validate_rejects_bad_sort_field() {
    let query = StatsQuery::new().sort_by("points; drop", SortDirection::Asc);
    assert_eq!(
        query.validate(),
        Err(StatsQueryError::InvalidField("points; drop".to_string()))
    );
}

#[test]
fn test_stats_query_validate_finds_bad_field_nested_in_groups() {
    let query = StatsQuery::new().filter(
        CayenneExpr::field("a")
            .eq(1)
            .and(!CayenneExpr::field("b c").is_in([1, 2])),
    );
    assert_eq!(
        query.validate(),
        Err(StatsQueryError::InvalidField("b c".to_string()))
    );
}

#[test]
fn test_stats_query_validate_rejects_non_finite_numbers() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let query = StatsQuery::new().filter(CayenneExpr::field("pointsPerGame").gt(value));
        assert_eq!(
            query.validate(),
            Err(StatsQueryError::NonFiniteValue {
                field: "pointsPerGame".to_string()
            })
        );
    }
    let in_list = StatsQuery::new().fact_filter(CayenneExpr::field("x").is_in([1.0, f64::NAN]));
    assert_eq!(
        in_list.validate(),
        Err(StatsQueryError::NonFiniteValue {
            field: "x".to_string()
        })
    );
}

#[test]
fn test_stats_query_validate_accepts_finite_floats() {
    let query = StatsQuery::new().filter(CayenneExpr::field("pointsPerGame").ge(1.25));
    assert_eq!(query.validate(), Ok(()));
}

#[test]
fn test_stats_query_validate_rejects_empty_in_list() {
    let empty: [i32; 0] = [];
    let query = StatsQuery::new().filter(CayenneExpr::field("teamId").is_in(empty));
    assert_eq!(
        query.validate(),
        Err(StatsQueryError::EmptyInList("teamId".to_string()))
    );
}

#[test]
fn test_stats_query_validate_rejects_zero_limit() {
    assert_eq!(
        StatsQuery::new().limit(0).validate(),
        Err(StatsQueryError::ZeroLimit)
    );
    assert_eq!(StatsQuery::new().limit(1).validate(), Ok(()));
}

#[test]
fn test_stats_query_error_messages_name_the_value() {
    assert!(StatsQueryError::InvalidField("a b".into())
        .to_string()
        .contains("\"a b\""));
    assert!(StatsQueryError::InvalidReport("x/y".into())
        .to_string()
        .contains("\"x/y\""));
    assert!(StatsQueryError::NonFiniteValue {
        field: "pts".into()
    }
    .to_string()
    .contains("pts"));
    assert!(StatsQueryError::EmptyInList("teamId".into())
        .to_string()
        .contains("teamId"));
    assert!(StatsQueryError::ZeroLimit.to_string().contains("unlimited"));
}

// ===== validate_report =====

#[test]
fn test_validate_report_accepts_config_report_names() {
    for name in ["summary", "faceoffpercentages", "startedVsRelieved", "a_1"] {
        assert_eq!(validate_report(name), Ok(()), "{name}");
    }
}

#[test]
fn test_validate_report_rejects_path_like_names() {
    for name in ["", "../x", "a/b", "a b", "a?x=1", "1abc", "summary#"] {
        assert_eq!(
            validate_report(name),
            Err(StatsQueryError::InvalidReport(name.to_string())),
            "{name:?}"
        );
    }
}

// ===== with_default_sort =====

#[test]
fn test_with_default_sort_applies_keys_when_query_has_none() {
    let query = StatsQuery::new().season(Season::new(2024));

    let sorted = query.with_default_sort(&[
        ("points", SortDirection::Desc),
        ("playerId", SortDirection::Asc),
    ]);

    assert!(matches!(sorted, Cow::Owned(_)));
    assert_eq!(
        sorted.sort_keys(),
        [
            SortKey::new("points", SortDirection::Desc),
            SortKey::new("playerId", SortDirection::Asc),
        ]
    );
    assert_eq!(sorted.cayenne_exp(), "seasonId=20242025");
    assert!(query.sort_keys().is_empty(), "the original is untouched");
}

#[test]
fn test_with_default_sort_keeps_callers_sort() {
    let query = StatsQuery::new().sort_by("goals", SortDirection::Desc);

    let sorted = query.with_default_sort(&[("points", SortDirection::Desc)]);

    assert!(matches!(sorted, Cow::Borrowed(_)));
    assert_eq!(
        sorted.sort_keys(),
        [SortKey::new("goals", SortDirection::Desc)]
    );
}
