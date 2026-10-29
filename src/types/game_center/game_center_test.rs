use super::GameStory;
use crate::ids::GameId;

/// Chicago at New York, 2024-12-09 (live API response, trimmed to one goal
/// and its three stars).
const GAME_STORY_FIXTURE: &str = include_str!("../../../tests/fixtures/game_story.json");

#[test]
fn test_game_story_fixture_deserializes_three_star_names() {
    let story: GameStory = serde_json::from_str(GAME_STORY_FIXTURE).unwrap();
    let summary = story.summary.expect("fixture includes a game summary");

    assert_eq!(story.id, GameId::new(2024020444));
    assert_eq!(summary.three_stars.len(), 3);
    assert_eq!(summary.three_stars[0].name, "T. Hall");
    assert_eq!(summary.three_stars[1].name, "A. Soderblom");

    let goal = &summary.scoring[0].goals[0];
    assert_eq!(goal.name.default, "T. Bertuzzi");
}

mod play_by_play_helpers {
    use super::super::{GameSituation, PlayByPlay, PlayEventType};
    use crate::ids::{PlayerId, TeamId};

    /// PHI 3 @ BUF 1, 2026-10-21: three regulation periods, no overtime.
    const PLAY_BY_PLAY_FIXTURE: &str =
        include_str!("../../../tests/fixtures/play_by_play/regular_20262027.json");
    const PLAYS: usize = 286;
    const PHILADELPHIA: i64 = 4;
    const BUFFALO: i64 = 7;
    /// Cole Knuble (PHI), who scored twice.
    const COLE_KNUBLE: i64 = 8483787;

    fn play_by_play() -> PlayByPlay {
        serde_json::from_str(PLAY_BY_PLAY_FIXTURE).unwrap()
    }

    #[test]
    fn test_recent_plays_most_recent_first() {
        let pbp = play_by_play();
        let recent: Vec<_> = pbp
            .recent_plays(3)
            .iter()
            .map(|play| (play.event_id, play.type_desc_key))
            .collect();

        assert_eq!(pbp.plays.len(), PLAYS);
        assert_eq!(
            recent,
            [
                (571, PlayEventType::GameEnd),
                (567, PlayEventType::PeriodEnd),
                (1069, PlayEventType::BlockedShot),
            ]
        );
        assert_eq!(pbp.recent_plays(PLAYS + 1).len(), PLAYS);
    }

    #[test]
    fn test_goals() {
        let pbp = play_by_play();
        let goals: Vec<_> = pbp
            .goals()
            .iter()
            .map(|goal| {
                let details = goal.details.as_ref().unwrap();
                (goal.event_id, details.scoring_player_id.unwrap())
            })
            .collect();

        assert_eq!(
            goals,
            [
                (501, PlayerId::new(COLE_KNUBLE)),
                (460, PlayerId::new(8481533)),
                (809, PlayerId::new(8475722)),
                (857, PlayerId::new(COLE_KNUBLE)),
            ]
        );
    }

    #[test]
    fn test_penalties() {
        let pbp = play_by_play();
        let penalties = pbp.penalties();

        assert_eq!(penalties.len(), 11);
        assert!(penalties
            .iter()
            .all(|play| play.type_desc_key == PlayEventType::Penalty));
        assert_eq!(penalties[0].event_id, 84);
    }

    #[test]
    fn test_shots_count_every_shot_attempt() {
        let pbp = play_by_play();
        let count = |kind| {
            pbp.shots()
                .iter()
                .filter(|play| play.type_desc_key == kind)
                .count()
        };

        assert_eq!(pbp.shots().len(), 109);
        assert_eq!(count(PlayEventType::ShotOnGoal), 50);
        assert_eq!(count(PlayEventType::BlockedShot), 30);
        assert_eq!(count(PlayEventType::MissedShot), 25);
        assert_eq!(count(PlayEventType::Goal), 4);
    }

    #[test]
    fn test_plays_in_period() {
        let pbp = play_by_play();

        assert_eq!(pbp.plays_in_period(1).len(), 110);
        assert_eq!(pbp.plays_in_period(2).len(), 90);
        assert_eq!(pbp.plays_in_period(3).len(), 86);
        assert!(pbp.plays_in_period(4).is_empty());
        assert!(pbp
            .plays_in_period(2)
            .iter()
            .all(|play| play.period_descriptor.number == 2));
    }

    #[test]
    fn test_get_player() {
        let pbp = play_by_play();
        let player = pbp.get_player(COLE_KNUBLE).unwrap();

        assert_eq!(player.last_name.default, "Knuble");
        assert_eq!(player.team_id, TeamId::new(PHILADELPHIA));
        assert_eq!(player.sweater_number, 54);
        assert!(pbp.get_player(1).is_none());
    }

    #[test]
    fn test_team_roster() {
        let pbp = play_by_play();
        let flyers = pbp.team_roster(PHILADELPHIA);

        assert_eq!(flyers.len(), 20);
        assert!(flyers
            .iter()
            .all(|spot| spot.team_id == TeamId::new(PHILADELPHIA)));
        assert_eq!(pbp.team_roster(BUFFALO).len(), 20);
        assert!(pbp.team_roster(1).is_empty());
    }

    /// The final play, with Buffalo's goalie pulled: `1560`.
    #[test]
    fn test_current_situation() {
        let situation = play_by_play().current_situation().unwrap();

        assert_eq!(
            situation,
            GameSituation {
                away_goalie_in: true,
                away_skaters: 5,
                home_skaters: 6,
                home_goalie_in: false,
            }
        );
    }

    #[test]
    fn test_current_situation_without_plays() {
        let mut pbp = play_by_play();
        pbp.plays.clear();

        assert!(pbp.current_situation().is_none());
        assert!(pbp.recent_plays(1).is_empty());
    }
}

mod landing {
    use super::super::{GameMatchup, ThreeStar};
    use crate::date::Season;
    use crate::ids::{GameId, PlayerId, TeamId};
    use crate::types::{GameState, GameType};

    /// PHI 3 @ BUF 1, 2026-10-21.
    const LANDING_FIXTURE: &str =
        include_str!("../../../tests/fixtures/landing/regular_20262027.json");

    #[test]
    fn test_landing_fixture_deserializes() {
        let landing: GameMatchup = serde_json::from_str(LANDING_FIXTURE).unwrap();

        assert_eq!(landing.id, GameId::new(2026020152));
        assert_eq!(landing.season, Season::new(2026));
        assert_eq!(landing.game_type, GameType::RegularSeason);
        assert_eq!(landing.game_state, GameState::Off);
        assert_eq!(landing.venue.default, "KeyBank Center");
        assert_eq!(landing.max_periods, Some(5));
        assert!(landing.ot_in_use && landing.shootout_in_use && !landing.ties_in_use);
        assert_eq!(landing.clock.unwrap().seconds_remaining, 0);
        let summary = landing.summary.unwrap();
        assert_eq!(summary.scoring.len(), 3);
        assert_eq!(summary.penalties.len(), 3);
    }

    #[test]
    fn test_landing_fixture_matchup_teams() {
        let landing: GameMatchup = serde_json::from_str(LANDING_FIXTURE).unwrap();
        let (away, home) = (&landing.away_team, &landing.home_team);

        assert_eq!(away.id, TeamId::new(4));
        assert_eq!(away.abbrev, "PHI");
        assert_eq!(away.common_name.default, "Flyers");
        assert_eq!(away.place_name.default, "Philadelphia");
        assert_eq!(away.place_name_with_preposition.default, "Philadelphia");
        assert_eq!((away.score, away.sog), (3, Some(26)));
        assert_eq!(home.abbrev, "BUF");
        assert_eq!((home.score, home.sog), (1, Some(28)));
        assert!(home.logo.ends_with(".svg"));
    }

    /// The landing response sends `threeStars[].name` as a localized object,
    /// unlike the game story's plain string.
    #[test]
    fn test_landing_fixture_three_star_names_are_localized() {
        let landing: GameMatchup = serde_json::from_str(LANDING_FIXTURE).unwrap();
        let stars: Vec<(PlayerId, &str)> = landing
            .summary
            .as_ref()
            .unwrap()
            .three_stars
            .iter()
            .map(|star| (star.player_id, star.name.as_str()))
            .collect();

        assert_eq!(
            stars,
            [
                (PlayerId::new(8483787), "C. Knuble"),
                (PlayerId::new(8478435), "D. Vladar"),
                (PlayerId::new(8475722), "J. Zucker"),
            ]
        );
    }

    #[test]
    fn test_three_star_name_rejects_other_shapes() {
        let json =
            r#"{"star":1,"playerId":1,"teamAbbrev":"PHI","headshot":"","name":42,"sweaterNo":1}"#;

        assert!(serde_json::from_str::<ThreeStar>(json).is_err());
    }
}

mod playoffs {
    use super::super::{GameMatchup, GameStory, PlayByPlay};
    use crate::ids::GameId;
    use crate::types::GameType;

    /// FLA 5 @ EDM 2, 2025-06-14, Stanley Cup Final game 5: no `maxPeriods`,
    /// since playoff overtime is unlimited.
    const PLAY_BY_PLAY_FIXTURE: &str =
        include_str!("../../../tests/fixtures/play_by_play/playoffs_20242025.json");
    const LANDING_FIXTURE: &str =
        include_str!("../../../tests/fixtures/landing/playoffs_20242025.json");
    const GAME_STORY_FIXTURE: &str =
        include_str!("../../../tests/fixtures/game_story/playoffs_20242025.json");
    const GAME_ID: i64 = 2024030415;
    const REGULATION_PERIODS: i32 = 3;

    #[test]
    fn test_playoff_play_by_play_has_no_max_periods() {
        let pbp: PlayByPlay = serde_json::from_str(PLAY_BY_PLAY_FIXTURE).unwrap();

        assert_eq!(pbp.id, GameId::new(GAME_ID));
        assert_eq!(pbp.game_type, GameType::Playoffs);
        assert_eq!(pbp.max_periods, None);
        assert_eq!(pbp.reg_periods, REGULATION_PERIODS);
        assert_eq!(
            pbp.period_descriptor.max_regulation_periods,
            REGULATION_PERIODS
        );
    }

    #[test]
    fn test_playoff_landing_has_no_max_periods() {
        let landing: GameMatchup = serde_json::from_str(LANDING_FIXTURE).unwrap();

        assert_eq!(landing.id, GameId::new(GAME_ID));
        assert_eq!(landing.game_type, GameType::Playoffs);
        assert_eq!(landing.max_periods, None);
        assert_eq!(landing.reg_periods, REGULATION_PERIODS);
        assert!(landing.ot_in_use && !landing.shootout_in_use);
    }

    #[test]
    fn test_playoff_game_story_has_no_max_periods() {
        let story: GameStory = serde_json::from_str(GAME_STORY_FIXTURE).unwrap();

        assert_eq!(story.id, GameId::new(GAME_ID));
        assert_eq!(story.game_type, GameType::Playoffs);
        assert_eq!(story.max_periods, None);
        assert_eq!(story.reg_periods, REGULATION_PERIODS);
    }

    #[test]
    fn test_playoff_max_periods_omitted_when_serialized() {
        let landing: GameMatchup = serde_json::from_str(LANDING_FIXTURE).unwrap();
        let value = serde_json::to_value(&landing).unwrap();

        assert!(value.get("maxPeriods").is_none());
        let round_trip: GameMatchup = serde_json::from_value(value).unwrap();
        assert_eq!(round_trip, landing);
    }
}
