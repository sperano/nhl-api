//! Contract tests: every response recorded from the live API under
//! `tests/fixtures/<method>/<label>.json` deserializes as the type its
//! `Client` method returns, unless `tests/common/known_failures.rs` lists it,
//! and still drops the fields `tests/common/known_dropped.rs` lists.
//! No network access; re-record with the `capture_fixtures` test in
//! `tests/live_sweep.rs`.

mod common;

use common::known_dropped::KNOWN_DROPPED;
use common::known_failures::KNOWN_FAILURES;
use common::{cases, describe_parse_error, CurrentGames, KnownFailure, Outcome};
use std::collections::HashSet;
use std::path::Path;

#[test]
fn test_recorded_responses_deserialize() {
    let mut checked = 0;
    let mut unexpected = Vec::new();
    for case in cases(&CurrentGames::default()) {
        let Some(json) = case.fixture else {
            continue;
        };
        checked += 1;
        let result = (case.parse)(json)
            .map(drop)
            .map_err(|err| describe_parse_error(json, &err));
        match Outcome::classify(&case.name, result) {
            Outcome::Pass | Outcome::KnownFailure => {}
            Outcome::KnownFailurePassed => unexpected.push(format!(
                "{} now deserializes: remove it from KNOWN_FAILURES",
                case.name
            )),
            Outcome::Failure(err) => unexpected.push(format!("{}: {err}", case.name)),
        }
    }

    assert!(checked > 0, "no recorded fixtures found");
    assert!(
        unexpected.is_empty(),
        "{} of {checked} recorded responses:\n{}",
        unexpected.len(),
        unexpected.join("\n")
    );
}

/// Deserializing into a type that ignores a field the API renamed still
/// succeeds, so each such field is pinned as dropped until its type is fixed.
#[test]
fn test_known_dropped_fields_are_still_dropped() {
    let cases = cases(&CurrentGames::default());
    let mut unexpected = Vec::new();
    for known in KNOWN_DROPPED {
        let Some((parse, json)) = cases
            .iter()
            .find(|case| case.name == known.case)
            .and_then(|case| case.fixture.map(|json| (case.parse, json)))
        else {
            unexpected.push(format!("{} is not a recorded case", known.case));
            continue;
        };
        let dropped = match parse(json) {
            Ok(dropped) => dropped,
            Err(err) => {
                unexpected.push(format!("{}: {err}", known.case));
                continue;
            }
        };
        for field in known
            .fields
            .iter()
            .filter(|field| !dropped.contains(**field))
        {
            unexpected.push(format!(
                "{}: `{field}` is no longer dropped: remove it from KNOWN_DROPPED",
                known.case
            ));
        }
    }
    assert!(
        unexpected.is_empty(),
        "{} unexpected KNOWN_DROPPED result(s):\n{}",
        unexpected.len(),
        unexpected.join("\n")
    );
}

#[test]
fn test_case_names_are_unique() {
    let mut seen = HashSet::new();
    for case in cases(&CurrentGames::default()) {
        assert!(
            seen.insert(case.name.clone()),
            "duplicate case {}",
            case.name
        );
    }
}

/// Entries for cases with no recorded fixture are checked by the live sweep.
#[test]
fn test_known_failures_name_existing_cases() {
    let names: HashSet<String> = cases(&CurrentGames::default())
        .into_iter()
        .map(|case| case.name)
        .collect();
    for known in KNOWN_FAILURES {
        assert!(
            names.contains(known.case),
            "KNOWN_FAILURES lists {}, which is not a case",
            known.case
        );
    }
}

#[test]
fn test_outcome_classify() {
    let known = KnownFailure {
        case: "boxscore/example",
        error: "missing field `score`",
    };
    let classify = |result: Result<(), &str>, known| {
        Outcome::classify_with(result.map_err(str::to_string), known)
    };

    assert_eq!(
        classify(
            Err("parsing: missing field `score` at line 1"),
            Some(&known)
        ),
        Outcome::KnownFailure
    );
    assert_eq!(classify(Ok(()), Some(&known)), Outcome::KnownFailurePassed);
    assert!(matches!(
        classify(Err("another error"), Some(&known)),
        Outcome::Failure(_)
    ));
    assert_eq!(classify(Ok(()), None), Outcome::Pass);
    assert_eq!(
        classify(Err("boom"), None),
        Outcome::Failure("boom".to_string())
    );
}

/// A fixture file no case references (left behind by a renamed or removed
/// case) would never be checked.
#[test]
fn test_every_recorded_file_belongs_to_a_case() {
    let recorded: HashSet<String> = cases(&CurrentGames::default())
        .into_iter()
        .filter(|case| case.fixture.is_some())
        .map(|case| format!("{}.json", case.name))
        .collect();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    for method_dir in subdirectories(&root) {
        let method = method_dir
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        for file in std::fs::read_dir(&method_dir).unwrap() {
            let file = file.unwrap().file_name().to_string_lossy().into_owned();
            let name = format!("{method}/{file}");
            assert!(
                recorded.contains(&name),
                "tests/fixtures/{name} is not the fixture of any case in tests/common/matrix.rs"
            );
        }
    }
}

/// Directories directly under `root`. Files directly under `root` are older
/// fixtures loaded by the unit tests in `src/`.
fn subdirectories(root: &Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(root)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect()
}
