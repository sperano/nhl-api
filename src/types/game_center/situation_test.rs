use super::GameSituation;

#[test]
fn test_game_situation_from_code() {
    let situation = GameSituation::from_code("1551").unwrap();
    assert_eq!(situation.away_skaters, 5);
    assert_eq!(situation.home_skaters, 5);
    assert!(situation.away_goalie_in);
    assert!(situation.home_goalie_in);
    assert!(situation.is_even_strength());
    assert!(!situation.is_away_power_play());
    assert!(!situation.is_home_power_play());
    assert!(!situation.is_empty_net());
    assert_eq!(situation.strength_description(), "5v5");
}

#[test]
fn test_game_situation_power_play() {
    let away_power_play = GameSituation::from_code("1541").unwrap();
    assert!(away_power_play.is_away_power_play());
    assert!(!away_power_play.is_home_power_play());
    assert!(!away_power_play.is_even_strength());
    assert_eq!(away_power_play.strength_description(), "5v4 PP");

    let home_power_play = GameSituation::from_code("1451").unwrap();
    assert!(!home_power_play.is_away_power_play());
    assert!(home_power_play.is_home_power_play());
    assert!(!home_power_play.is_even_strength());
    assert_eq!(home_power_play.strength_description(), "4v5 PP");
}

#[test]
fn test_game_situation_empty_net_even_strength() {
    let away_empty_net = GameSituation::from_code("0651").unwrap();
    assert!(away_empty_net.is_empty_net());
    assert!(away_empty_net.is_even_strength());
    assert!(!away_empty_net.is_away_power_play());
    assert!(!away_empty_net.is_home_power_play());
    assert_eq!(away_empty_net.strength_description(), "6v5 EN");

    let home_empty_net = GameSituation::from_code("1560").unwrap();
    assert!(home_empty_net.is_empty_net());
    assert!(home_empty_net.is_even_strength());
    assert!(!home_empty_net.is_away_power_play());
    assert!(!home_empty_net.is_home_power_play());
    assert_eq!(home_empty_net.strength_description(), "5v6 EN");
}

#[test]
fn test_game_situation_power_play_with_pulled_goalie() {
    let away_power_play = GameSituation::from_code("0641").unwrap();
    assert!(away_power_play.is_away_power_play());
    assert!(!away_power_play.is_home_power_play());
    assert!(!away_power_play.is_even_strength());

    let home_power_play = GameSituation::from_code("1460").unwrap();
    assert!(!home_power_play.is_away_power_play());
    assert!(home_power_play.is_home_power_play());
    assert!(!home_power_play.is_even_strength());
}

#[test]
fn test_game_situation_equal_raw_counts_with_pulled_goalie() {
    let away_short_handed = GameSituation::from_code("0551").unwrap();
    assert!(!away_short_handed.is_even_strength());
    assert!(!away_short_handed.is_away_power_play());
    assert!(away_short_handed.is_home_power_play());
}

#[test]
fn test_game_situation_invalid_code() {
    assert!(GameSituation::from_code("155").is_none());
    assert!(GameSituation::from_code("15512").is_none());
    assert!(GameSituation::from_code("abcd").is_none());
    assert!(GameSituation::from_code("a55b").is_none());
    assert!(GameSituation::from_code("1991").is_none());
    assert!(GameSituation::from_code("2551").is_none());
}

#[test]
fn test_game_situation_three_on_three_with_pulled_goalie() {
    let situation = GameSituation::from_code("0331").unwrap();
    assert!(!situation.is_even_strength());
    assert!(!situation.is_away_power_play());
    assert!(situation.is_home_power_play());
}

#[test]
fn test_game_situation_display() {
    let situation = GameSituation::from_code("1551").unwrap();
    assert_eq!(format!("{}", situation), "5v5");
}
