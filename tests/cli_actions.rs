use std::{
    env, fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use pretty_assertions::assert_eq;
use serde_json::json;
use ticktick_cli::{
    api::HttpMethod,
    cli::{Action, Cli, JsonInput},
    error::TickTickError,
};

fn unique_temp_file(name: &str, contents: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = env::temp_dir().join(format!("ticktick-cli-{name}-{nanos}.json"));
    fs::write(&path, contents).unwrap();
    path
}

#[test]
fn oauth_authorize_command_returns_pretty_json_action() {
    let cli = Cli::parse_from([
        "ticktick",
        "--client-id",
        "client-123",
        "oauth",
        "authorize",
        "--redirect-uri",
        "https://localhost/callback",
        "--state",
        "opaque-state",
    ]);

    let action = cli.into_action().unwrap();
    match action {
        Action::PrintJson(value) => assert_eq!(
            value,
            json!({
                "authorize_url": "https://ticktick.com/oauth/authorize?client_id=client-123&scope=tasks%3Aread+tasks%3Awrite&state=opaque-state&redirect_uri=https%3A%2F%2Flocalhost%2Fcallback&response_type=code"
            })
        ),
        other => panic!("expected PrintJson action, got {other:?}"),
    }
}

#[test]
fn oauth_exchange_command_requires_client_secret() {
    let cli = Cli::parse_from([
        "ticktick",
        "--client-id",
        "client-123",
        "oauth",
        "exchange",
        "--code",
        "code-123",
        "--redirect-uri",
        "https://localhost/callback",
    ]);

    let error = cli.into_action().unwrap_err();
    assert!(matches!(error, TickTickError::MissingClientSecret));
}

#[test]
fn into_request_uses_default_token_when_cli_token_is_missing() {
    let cli = Cli::parse_from([
        "ticktick",
        "task",
        "get",
        "--project-id",
        "proj-1",
        "--task-id",
        "task-1",
    ]);

    let request = cli.into_request(Some("fallback-token".into())).unwrap();
    assert_eq!(request.method, HttpMethod::Get);
    assert_eq!(request.path, "/open/v1/project/proj-1/task/task-1");
    assert_eq!(request.token.as_deref(), Some("fallback-token"));
}

#[test]
fn into_request_errors_when_no_token_is_available() {
    let cli = Cli::parse_from(["ticktick", "project", "list"]);

    let error = cli.into_request(None).unwrap_err();
    assert!(matches!(error, TickTickError::MissingAccessToken));
}

#[test]
fn cli_parses_project_and_focus_commands_into_expected_requests() {
    let project_create = Cli::parse_from([
        "ticktick",
        "--token",
        "tok",
        "project",
        "create",
        "--json",
        r#"{"name":"Inbox"}"#,
    ])
    .into_request(None)
    .unwrap();
    assert_eq!(project_create.method, HttpMethod::Post);
    assert_eq!(project_create.path, "/open/v1/project");
    assert_eq!(project_create.body, Some(json!({"name": "Inbox"})));

    let focus_list = Cli::parse_from([
        "ticktick", "--token", "tok", "focus", "list", "--from", "20260401", "--to", "20260407",
        "--type", "1",
    ])
    .into_request(None)
    .unwrap();
    assert_eq!(focus_list.method, HttpMethod::Get);
    assert_eq!(focus_list.path, "/open/v1/focus");
    assert_eq!(
        focus_list.query,
        vec![
            ("from".into(), "20260401".into()),
            ("to".into(), "20260407".into()),
            ("type".into(), "1".into()),
        ]
    );
}

#[test]
fn cli_parses_habit_and_task_json_commands_from_files() {
    let habit_path = unique_temp_file("habit", r#"{"name":"Read"}"#);
    let task_path = unique_temp_file(
        "task-filter",
        r#"{"projectIds":["p1"],"tag":["deep"],"status":[0]}"#,
    );

    let habit_request = Cli::parse_from([
        "ticktick",
        "--token",
        "tok",
        "habit",
        "create",
        "--json-file",
        habit_path.to_str().unwrap(),
    ])
    .into_request(None)
    .unwrap();
    assert_eq!(habit_request.path, "/open/v1/habit");
    assert_eq!(habit_request.body, Some(json!({"name": "Read"})));

    let task_request = Cli::parse_from([
        "ticktick",
        "--token",
        "tok",
        "task",
        "filter",
        "--json-file",
        task_path.to_str().unwrap(),
    ])
    .into_request(None)
    .unwrap();
    assert_eq!(task_request.path, "/open/v1/task/filter");
    assert_eq!(
        task_request.body,
        Some(json!({"projectIds": ["p1"], "tag": ["deep"], "status": [0]}))
    );

    fs::remove_file(habit_path).unwrap();
    fs::remove_file(task_path).unwrap();
}

#[test]
fn json_input_reads_inline_and_file_json() {
    let input = JsonInput {
        json: Some(r#"{"projectId":"p1","title":"Ship tests"}"#.into()),
        json_file: None,
        json_stdin: false,
    };
    assert_eq!(
        input.read_required_json().unwrap(),
        json!({"projectId": "p1", "title": "Ship tests"})
    );

    let path = unique_temp_file("project", r#"{"name":"Errands"}"#);
    let file_input = JsonInput {
        json: None,
        json_file: Some(path.to_string_lossy().into_owned()),
        json_stdin: false,
    };
    assert_eq!(
        file_input.read_required_json().unwrap(),
        json!({"name": "Errands"})
    );
    fs::remove_file(path).unwrap();
}

#[test]
fn json_input_rejects_missing_conflicting_invalid_and_missing_file_cases() {
    let missing = JsonInput {
        json: None,
        json_file: None,
        json_stdin: false,
    }
    .read_required_json()
    .unwrap_err();
    assert!(matches!(missing, TickTickError::MissingJsonInput));

    let conflicting = JsonInput {
        json: Some("{}".into()),
        json_file: Some("/tmp/ignored.json".into()),
        json_stdin: false,
    }
    .read_required_json()
    .unwrap_err();
    assert!(matches!(conflicting, TickTickError::ConflictingJsonInput));

    let invalid = JsonInput {
        json: Some("{".into()),
        json_file: None,
        json_stdin: false,
    }
    .read_required_json()
    .unwrap_err();
    assert!(matches!(invalid, TickTickError::InvalidJson(_)));

    let missing_file = JsonInput {
        json: None,
        json_file: Some("/definitely/not/here.json".into()),
        json_stdin: false,
    }
    .read_required_json()
    .unwrap_err();
    match missing_file {
        TickTickError::ReadJsonFile { path, .. } => assert_eq!(path, "/definitely/not/here.json"),
        other => panic!("expected ReadJsonFile error, got {other:?}"),
    }
}
