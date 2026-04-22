use pretty_assertions::assert_eq;
use serde_json::json;
use ticktick_cli::{config::AuthUrlConfig, output::format_json};

#[test]
fn auth_url_includes_required_state_from_docs() {
    let cfg = AuthUrlConfig {
        client_id: "client-123".into(),
        scope: "tasks:read tasks:write".into(),
        state: "state-123".into(),
        redirect_uri: "https://localhost/callback?source=test".into(),
    };

    let url = cfg.build_url().unwrap();
    assert_eq!(
        url.as_str(),
        "https://ticktick.com/oauth/authorize?client_id=client-123&scope=tasks%3Aread+tasks%3Awrite&state=state-123&redirect_uri=https%3A%2F%2Flocalhost%2Fcallback%3Fsource%3Dtest&response_type=code"
    );
}

#[test]
fn format_json_pretty_prints_with_stable_indentation() {
    let rendered = format_json(&json!({
        "habit": "Read",
        "details": { "minutes": 30, "completed": true }
    }))
    .unwrap();

    assert!(rendered.contains("\n  \"details\": {\n"));
    assert!(rendered.contains("\n    \"minutes\": 30"));
    assert!(rendered.contains("\n    \"completed\": true"));
    assert!(rendered.ends_with("\n}"));
}
