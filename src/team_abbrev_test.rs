use super::*;

#[test]
fn test_team_abbrev_new_valid() {
    let abbrev = TeamAbbrev::new("MTL").unwrap();
    assert_eq!(abbrev.as_str(), "MTL");
    assert_eq!(abbrev.to_string(), "MTL");
    assert_eq!(abbrev.as_ref(), "MTL");
}

#[test]
fn test_team_abbrev_new_normalizes_lowercase() {
    assert_eq!(
        TeamAbbrev::new("mTl").unwrap(),
        TeamAbbrev::new("MTL").unwrap()
    );
}

#[test]
fn test_team_abbrev_new_rejects_bad_input() {
    for bad in [
        "", "MT", "MTLX", "MT1", "../", "MT/", "M?T", "M#T", "M%2", "M T", "ÉTÉ", "MÉ",
    ] {
        let err = TeamAbbrev::new(bad).unwrap_err();
        assert_eq!(err.value(), bad);
    }
}

#[test]
fn test_team_abbrev_error_message() {
    let err = TeamAbbrev::new("a/b").unwrap_err();
    assert_eq!(
        err.to_string(),
        r#"invalid team abbreviation "a/b": expected 3 ASCII letters"#
    );
}

#[test]
fn test_team_abbrev_from_str_and_try_from() {
    assert_eq!("tor".parse::<TeamAbbrev>().unwrap().as_str(), "TOR");
    assert_eq!(TeamAbbrev::try_from("BUF").unwrap().as_str(), "BUF");
    assert_eq!(
        TeamAbbrev::try_from(String::from("uta")).unwrap().as_str(),
        "UTA"
    );
    assert!(TeamAbbrev::try_from("TOOLONG").is_err());
}

#[test]
fn test_team_abbrev_serde_roundtrip() {
    let abbrev = TeamAbbrev::new("MMR").unwrap();
    let json = serde_json::to_string(&abbrev).unwrap();
    assert_eq!(json, r#""MMR""#);
    assert_eq!(serde_json::from_str::<TeamAbbrev>(&json).unwrap(), abbrev);
}

#[test]
fn test_team_abbrev_deserialize_rejects_invalid() {
    let err = serde_json::from_str::<TeamAbbrev>(r#""M/T""#).unwrap_err();
    assert!(err.to_string().contains("invalid team abbreviation"));
}
