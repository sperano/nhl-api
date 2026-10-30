use super::*;

fn field(name: &str) -> CayenneField {
    CayenneExpr::field(name)
}

// ===== CompareOp =====

#[test]
fn test_compare_op_renders_each_operator() {
    let cases = [
        (CompareOp::Eq, "="),
        (CompareOp::Ne, "!="),
        (CompareOp::Lt, "<"),
        (CompareOp::Le, "<="),
        (CompareOp::Gt, ">"),
        (CompareOp::Ge, ">="),
        (CompareOp::Like, " like "),
        (CompareOp::LikeIgnoreCase, " likeIgnoreCase "),
    ];
    for (op, expected) in cases {
        assert_eq!(op.as_str(), expected);
        assert_eq!(op.to_string(), expected);
    }
}

// ===== Comparisons =====

#[test]
fn test_cayenne_field_comparisons_render() {
    assert_eq!(field("points").eq(100).to_string(), "points=100");
    assert_eq!(field("points").ne(0).to_string(), "points!=0");
    assert_eq!(field("points").lt(10).to_string(), "points<10");
    assert_eq!(field("points").le(10).to_string(), "points<=10");
    assert_eq!(field("points").gt(10).to_string(), "points>10");
    assert_eq!(field("points").ge(100).to_string(), "points>=100");
    assert_eq!(
        field("lastName").like("Mc%").to_string(),
        "lastName like 'Mc%'"
    );
    assert_eq!(
        field("lastName").like_ignore_case("mc%").to_string(),
        "lastName likeIgnoreCase 'mc%'"
    );
}

#[test]
fn test_cayenne_compare_builds_compare_variant() {
    assert_eq!(
        field("goals").gt(50),
        CayenneExpr::Compare {
            field: "goals".to_string(),
            op: CompareOp::Gt,
            value: CayenneValue::Int(50),
        }
    );
}

// ===== Values =====

#[test]
fn test_cayenne_value_string_escapes_quote_and_backslash() {
    assert_eq!(
        field("skaterFullName").eq("Ryan O'Reilly").to_string(),
        r"skaterFullName='Ryan O\'Reilly'"
    );
    assert_eq!(
        CayenneValue::from(r"a\b").to_string(),
        r"'a\\b'",
        "a backslash is doubled"
    );
    assert_eq!(CayenneValue::from(r"\'").to_string(), r"'\\\''");
}

#[test]
fn test_cayenne_value_string_keeps_non_ascii_and_empty() {
    assert_eq!(CayenneValue::from("Montréal").to_string(), "'Montréal'");
    assert_eq!(CayenneValue::from("").to_string(), "''");
}

#[test]
fn test_cayenne_value_float_renders_without_exponent() {
    assert_eq!(CayenneValue::from(0.5).to_string(), "0.5");
    assert_eq!(CayenneValue::from(2.0).to_string(), "2");
    assert_eq!(
        CayenneValue::from(1e21).to_string(),
        "1000000000000000000000"
    );
}

#[test]
fn test_cayenne_value_from_integer_types() {
    assert_eq!(CayenneValue::from(-3i32), CayenneValue::Int(-3));
    assert_eq!(CayenneValue::from(3i64), CayenneValue::Int(3));
    assert_eq!(CayenneValue::from(3u8), CayenneValue::Int(3));
    assert_eq!(CayenneValue::from(3u16), CayenneValue::Int(3));
    assert_eq!(CayenneValue::from(u32::MAX), CayenneValue::Int(4294967295));
}

#[test]
fn test_cayenne_value_from_float_and_strings() {
    assert_eq!(CayenneValue::from(1.5), CayenneValue::Float(1.5));
    assert_eq!(CayenneValue::from("x"), CayenneValue::Str("x".to_string()));
    assert_eq!(
        CayenneValue::from("x".to_string()),
        CayenneValue::Str("x".to_string())
    );
}

#[test]
fn test_cayenne_value_from_crate_types() {
    assert_eq!(
        CayenneValue::from(Season::new(2024)),
        CayenneValue::Int(20242025)
    );
    assert_eq!(CayenneValue::from(GameType::Playoffs), CayenneValue::Int(3));
    assert_eq!(
        CayenneValue::from(GameId::new(2024020001)),
        CayenneValue::Int(2024020001)
    );
    assert_eq!(
        CayenneValue::from(PlayerId::new(8478402)),
        CayenneValue::Int(8478402)
    );
    assert_eq!(CayenneValue::from(TeamId::new(8)), CayenneValue::Int(8));
    assert_eq!(
        CayenneValue::from(FranchiseId::new(5)),
        CayenneValue::Int(5)
    );
}

#[test]
fn test_cayenne_value_is_finite() {
    assert!(CayenneValue::Int(1).is_finite());
    assert!(CayenneValue::Str("x".into()).is_finite());
    assert!(CayenneValue::Float(1.5).is_finite());
    assert!(!CayenneValue::Float(f64::NAN).is_finite());
    assert!(!CayenneValue::Float(f64::INFINITY).is_finite());
    assert!(!CayenneValue::Float(f64::NEG_INFINITY).is_finite());
}

// ===== In =====

#[test]
fn test_cayenne_in_renders_comma_separated_values() {
    assert_eq!(
        field("points").is_in([100, 121]).to_string(),
        "points in (100,121)"
    );
    assert_eq!(
        field("teamAbbrev").is_in(["MTL", "TOR"]).to_string(),
        "teamAbbrev in ('MTL','TOR')"
    );
    assert_eq!(field("points").is_in([100]).to_string(), "points in (100)");
}

#[test]
fn test_cayenne_in_accepts_typed_ids() {
    let teams = [TeamId::new(8), TeamId::new(10)];
    assert_eq!(field("teamId").is_in(teams).to_string(), "teamId in (8,10)");
}

// ===== Combinators =====

#[test]
fn test_cayenne_and_joins_with_and() {
    let expr = field("seasonId")
        .ge(20202021)
        .and(field("gameTypeId").eq(2));
    assert_eq!(expr.to_string(), "seasonId>=20202021 and gameTypeId=2");
}

#[test]
fn test_cayenne_and_flattens_into_existing_and() {
    let left = field("a").eq(1).and(field("b").eq(2));
    let right = field("c").eq(3).and(field("d").eq(4));

    let expr = left.and(right);

    let CayenneExpr::And(terms) = &expr else {
        panic!("expected And, got {expr:?}");
    };
    assert_eq!(terms.len(), 4);
    assert_eq!(expr.to_string(), "a=1 and b=2 and c=3 and d=4");
}

#[test]
fn test_cayenne_or_flattens_into_existing_or() {
    let expr = field("a").eq(1).or(field("b").eq(2)).or(field("c").eq(3));

    let CayenneExpr::Or(terms) = &expr else {
        panic!("expected Or, got {expr:?}");
    };
    assert_eq!(terms.len(), 3);
    assert_eq!(expr.to_string(), "a=1 or b=2 or c=3");
}

#[test]
fn test_cayenne_top_level_or_is_not_parenthesized() {
    let expr = field("a").eq(1).or(field("b").eq(2));
    assert_eq!(expr.to_string(), "a=1 or b=2");
}

#[test]
fn test_cayenne_and_wraps_or_child_in_parentheses() {
    let expr = field("gameTypeId").eq(2).and(
        field("positionCode")
            .eq("C")
            .or(field("positionCode").eq("L")),
    );
    assert_eq!(
        expr.to_string(),
        "gameTypeId=2 and (positionCode='C' or positionCode='L')"
    );
}

#[test]
fn test_cayenne_or_wraps_and_child_in_parentheses() {
    let expr = field("a").eq(1).and(field("b").eq(2)).or(field("c").eq(3));
    assert_eq!(expr.to_string(), "(a=1 and b=2) or c=3");
}

#[test]
fn test_cayenne_and_does_not_flatten_or_into_and() {
    let or = field("a").eq(1).or(field("b").eq(2));
    let expr = or.and(field("c").eq(3));
    assert_eq!(expr.to_string(), "(a=1 or b=2) and c=3");
}

#[test]
fn test_cayenne_not_wraps_inner_in_parentheses() {
    let inner = field("points").eq(0).or(field("points").eq(1));
    assert_eq!((!inner).to_string(), "not (points=0 or points=1)");
    assert_eq!((!field("points").eq(0)).to_string(), "not (points=0)");
}

#[test]
fn test_cayenne_not_nests_inside_and() {
    let expr = field("a").eq(1).and(!field("b").eq(2));
    assert_eq!(expr.to_string(), "a=1 and not (b=2)");
}

#[test]
fn test_cayenne_double_not_renders_both() {
    assert_eq!((!!field("a").eq(1)).to_string(), "not (not (a=1))");
}

// ===== Empty groups =====

#[test]
fn test_cayenne_empty_groups_render_empty() {
    assert_eq!(CayenneExpr::And(vec![]).to_string(), "");
    assert_eq!(CayenneExpr::Or(vec![]).to_string(), "");
    assert_eq!((!CayenneExpr::And(vec![])).to_string(), "");
}

#[test]
fn test_cayenne_empty_group_is_skipped_inside_another_group() {
    let expr = CayenneExpr::And(vec![
        CayenneExpr::Or(vec![]),
        field("a").eq(1),
        CayenneExpr::And(vec![]),
        field("b").eq(2),
    ]);
    assert_eq!(expr.to_string(), "a=1 and b=2");
}

#[test]
fn test_cayenne_single_term_or_in_and_has_no_parentheses() {
    let expr = CayenneExpr::And(vec![
        CayenneExpr::Or(vec![field("a").eq(1)]),
        field("b").eq(2),
    ]);
    assert_eq!(expr.to_string(), "a=1 and b=2");
}

// ===== Conjunction =====

#[test]
fn test_render_conjunction_parenthesizes_or_terms() {
    let terms = [field("a").eq(1), field("b").eq(2).or(field("c").eq(3))];
    assert_eq!(render_conjunction(&terms), "a=1 and (b=2 or c=3)");
    assert_eq!(render_conjunction(&[]), "");
}

// ===== Visitors =====

#[test]
fn test_cayenne_visit_leaves_reports_every_field() {
    let expr = field("a")
        .eq(1)
        .and(!field("b").is_in([2, 3]))
        .and(field("c").eq(4).or(field("d").eq(5)));
    let mut seen = Vec::new();

    expr.visit_leaves(&mut |name, values| seen.push((name.to_string(), values.len())));

    assert_eq!(
        seen,
        [("a", 1), ("b", 2), ("c", 1), ("d", 1)].map(|(n, c)| (n.to_string(), c))
    );
}

#[test]
fn test_cayenne_empty_in_field_finds_nested_empty_list() {
    let empty: [i32; 0] = [];
    let expr = field("a").eq(1).and(!field("b").is_in(empty));
    assert_eq!(expr.empty_in_field(), Some("b"));
    assert_eq!(field("a").is_in([1]).empty_in_field(), None);
    assert_eq!(field("a").eq(1).empty_in_field(), None);
}
