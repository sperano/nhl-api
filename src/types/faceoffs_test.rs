use super::*;
use crate::ids::PlayerId;
use crate::types::boxscore::{SkaterStats, TeamGameStats, TeamPlayerStats};
use crate::types::common::LocalizedString;
use crate::types::enums::Position;

/// Rangers 1-2 Blackhawks, game 2024020444 (live play-by-play, trimmed to
/// every faceoff event, three other events, and the faceoff takers' roster
/// spots).
const PLAY_BY_PLAY_FIXTURE: &str = include_str!("../../tests/fixtures/play_by_play_faceoffs.json");

const CHICAGO: TeamId = TeamId::new(16);
const NY_RANGERS: TeamId = TeamId::new(3);
const TEAM_NOT_IN_GAME: TeamId = TeamId::new(999);

/// Faceoff totals the right-rail `teamGameStats.faceoffWins` category reports
/// for the fixture game: `"13/35"` away, `"22/35"` home.
const FIXTURE_FACEOFFS: GameFaceoffs = GameFaceoffs {
    away: FaceoffTotals {
        wins: 13,
        total: 35,
    },
    home: FaceoffTotals {
        wins: 22,
        total: 35,
    },
};

/// Faceoffs in the fixture won by a left winger (L. Reichel and P. Maroon,
/// both Chicago), whose boxscore `faceoffWinningPctg` is 0.25 and 0.5.
const LEFT_WING_FACEOFF_WINS: i32 = 2;

const FLOAT_TOLERANCE: f64 = 1e-9;

fn fixture() -> PlayByPlay {
    serde_json::from_str(PLAY_BY_PLAY_FIXTURE).unwrap()
}

fn is_faceoff(play: &PlayEvent) -> bool {
    play.type_desc_key == PlayEventType::Faceoff
}

fn set_owner(play: &mut PlayEvent, owner: Option<TeamId>) {
    play.details.as_mut().unwrap().event_owner_team_id = owner;
}

/// P. Maroon's boxscore line in the fixture game: a left winger with a
/// nonzero faceoff percentage.
fn left_winger_with_faceoffs() -> SkaterStats {
    SkaterStats {
        player_id: PlayerId::new(8474034),
        sweater_number: 77,
        name: LocalizedString {
            default: "P. Maroon".to_string(),
        },
        position: Some(Position::LeftWing),
        goals: 0,
        assists: 0,
        points: 0,
        plus_minus: -1,
        pim: 0,
        hits: 2,
        power_play_goals: 0,
        sog: 0,
        faceoff_winning_pctg: 0.5,
        toi: "12:00".to_string(),
        blocked_shots: 1,
        shifts: 15,
        giveaways: 0,
        takeaways: 0,
    }
}

#[test]
fn test_faceoff_totals_match_right_rail() {
    assert_eq!(fixture().faceoff_totals(), FIXTURE_FACEOFFS);
}

#[test]
fn test_faceoff_totals_count_left_winger_wins() {
    let mut pbp = fixture();
    let roster = pbp.roster_spots.clone();
    let won_by_left_wing = |play: &PlayEvent| {
        let winner = play.details.as_ref().and_then(|d| d.winning_player_id);
        roster
            .iter()
            .any(|spot| Some(spot.player_id) == winner && spot.position == Some(Position::LeftWing))
    };
    pbp.plays
        .retain(|play| is_faceoff(play) && won_by_left_wing(play));

    let faceoffs = pbp.faceoff_totals();

    assert_eq!(faceoffs.away.wins, LEFT_WING_FACEOFF_WINS);
    assert_eq!(faceoffs.away.total, LEFT_WING_FACEOFF_WINS);
    assert_eq!(
        faceoffs.home,
        FaceoffTotals {
            wins: 0,
            total: LEFT_WING_FACEOFF_WINS
        }
    );
}

#[test]
fn test_faceoff_totals_skip_faceoffs_without_known_winner() {
    let mut pbp = fixture();
    // Take Rangers wins: clear the owner of one, credit another to a team
    // that is not in the game.
    let mut rangers_wins = pbp.plays.iter_mut().filter(|play| {
        is_faceoff(play) && play.details.as_ref().unwrap().event_owner_team_id == Some(NY_RANGERS)
    });
    set_owner(rangers_wins.next().unwrap(), None);
    set_owner(rangers_wins.next().unwrap(), Some(TEAM_NOT_IN_GAME));
    const UNKNOWN_WINNERS: i32 = 2;

    let totals = pbp.faceoff_totals();

    let counted = FIXTURE_FACEOFFS.home.total - UNKNOWN_WINNERS;
    assert_eq!(
        totals.away,
        FaceoffTotals {
            wins: FIXTURE_FACEOFFS.away.wins,
            total: counted
        }
    );
    assert_eq!(
        totals.home,
        FaceoffTotals {
            wins: FIXTURE_FACEOFFS.home.wins - UNKNOWN_WINNERS,
            total: counted
        }
    );
}

#[test]
fn test_faceoff_totals_ignore_other_events() {
    let mut pbp = fixture();
    pbp.plays.retain(|play| !is_faceoff(play));
    for play in &mut pbp.plays {
        if play.details.is_some() {
            set_owner(play, Some(CHICAGO));
        }
    }
    assert!(pbp.plays.iter().any(|play| play.details.is_some()));

    assert_eq!(pbp.faceoff_totals(), GameFaceoffs::default());
}

#[test]
fn test_faceoff_totals_percentage() {
    let percentage = FIXTURE_FACEOFFS.away.percentage().unwrap();
    assert!((percentage - 13.0 / 35.0 * 100.0).abs() < FLOAT_TOLERANCE);
    assert_eq!(FaceoffTotals::default().percentage(), None);
}

#[test]
fn test_team_game_stats_does_not_estimate_faceoffs() {
    // A boxscore faceoff percentage carries no faceoff count, for centers and
    // wingers alike, so nothing may be derived from it (or from shifts).
    let team_stats = TeamPlayerStats {
        forwards: vec![left_winger_with_faceoffs()],
        defense: vec![],
        goalies: vec![],
    };

    let game_stats = TeamGameStats::from_team_player_stats(&team_stats);

    assert_eq!(game_stats.faceoffs, None);
    assert_eq!(game_stats.faceoff_percentage(), None);
}

#[test]
fn test_team_game_stats_with_play_by_play_faceoffs() {
    let team_stats = TeamPlayerStats {
        forwards: vec![left_winger_with_faceoffs()],
        defense: vec![],
        goalies: vec![],
    };
    let faceoffs = fixture().faceoff_totals();

    let game_stats =
        TeamGameStats::from_team_player_stats(&team_stats).with_faceoffs(faceoffs.away);

    assert_eq!(game_stats.faceoffs, Some(FIXTURE_FACEOFFS.away));
    assert_eq!(
        game_stats.faceoff_percentage(),
        FIXTURE_FACEOFFS.away.percentage()
    );
    assert_eq!(game_stats.hits, 2);
    assert_eq!(game_stats.blocked_shots, 1);
}

#[test]
fn test_team_game_stats_zero_faceoffs_percentage() {
    let game_stats = TeamGameStats::default().with_faceoffs(FaceoffTotals::default());
    assert_eq!(game_stats.faceoff_percentage(), None);
}
