//! Source-level contract tests for the AttoLang SDK preview.
//! Does not claim Atto compiler execution or HTTP/JSON implementation.

use evgl_client::Client;

const ATTO: &str = include_str!("../../atto/src/routes.atto");
const MATRIX: &str = include_str!("../../matrix.json");
const ZED: &str = include_str!("../../../.zpkg.toml");

fn path_literal(name: &str) -> String {
    let prefix = format!("pub fnc {name}(): string {{ return \"");
    let line = ATTO
        .lines()
        .find(|line| line.starts_with(&prefix))
        .unwrap_or_else(|| panic!("missing Atto path helper {name}"));
    let remainder = line
        .strip_prefix(&prefix)
        .expect("path helper declaration");
    let value = remainder
        .strip_suffix("\"; }")
        .expect("path literal must be a single static string");
    assert!(value.starts_with('/'), "Atto route {name} must be absolute");
    assert!(!value.contains(".."), "Atto route {name} cannot traverse paths");
    assert!(!value.contains('{'), "Atto routes must not interpolate unchecked identifiers");
    value.to_owned()
}

#[test]
fn atto_target_is_registered_once_in_canonical_manifests() {
    assert_eq!(MATRIX.matches("\"attolang\"").count(), 1);
    assert!(MATRIX.contains("\"dir\": \"clients/atto\""));
    assert!(MATRIX.contains("\"sentinel\": \"src/routes.atto\""));
    assert_eq!(ZED.matches("[targets.atto]").count(), 1);
    assert!(ZED.contains("dir = \"clients/atto\""));
}

#[test]
fn atto_literal_routes_match_the_rust_sdk() {
    let root = "https://api.example.invalid";
    let client = Client::new(root).expect("secure fixture URL");

    for (name, expected_url) in [
        ("health_path", client.health().url),
        ("config_path", client.config().url),
        ("emit_event_path", client.events().url),
        ("emit_alert_path", client.alerts().url),
        ("providers_path", format!("{root}/v1/providers")),
        ("connections_path", format!("{root}/v1/connections")),
        ("list_events_path", format!("{root}/v1/events")),
        ("create_event_path", format!("{root}/v1/events")),
    ] {
        assert_eq!(format!("{root}{}", path_literal(name)), expected_url, "{name}");
    }

    assert_eq!(
        ATTO.lines()
            .filter(|line| line.starts_with("pub fnc ") && line.contains("_path():"))
            .count(),
        8,
        "add a parity assertion for every new static route"
    );
}

#[test]
fn atto_auth_helper_preserves_security_boundary() {
    assert!(ATTO.contains(
        "pub fnc authorization_value(string token): string { return \"Bearer \" + token; }"
    ));
    assert!(ATTO.contains("Consumers must validate destination origins"));
    assert!(ATTO.contains("Never log or persist the returned value"));
    for dynamic in ["job_path", "cross_post_path", "start_oauth_path"] {
        assert!(
            !ATTO.contains(&format!("fnc {dynamic}(")),
            "dynamic route must validate identifiers before admission"
        );
    }
}
