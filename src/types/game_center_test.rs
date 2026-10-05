use super::GameStory;
use crate::ids::GameId;

/// Chicago at New York, 2024-12-09 (live API response, trimmed to one goal
/// and its three stars).
const GAME_STORY_FIXTURE: &str = include_str!("../../tests/fixtures/game_story.json");

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
