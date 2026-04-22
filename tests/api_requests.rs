use pretty_assertions::assert_eq;
use serde_json::json;
use ticktick_cli::api::{ApiClient, ApiRequest, HttpMethod};

fn assert_request(
    request: ApiRequest,
    method: HttpMethod,
    path: &str,
    query: Vec<(&str, &str)>,
    body: Option<serde_json::Value>,
    form: Vec<(&str, &str)>,
    token: Option<&str>,
    basic_auth: Option<&str>,
) {
    assert_eq!(request.method, method);
    assert_eq!(request.path, path);
    assert_eq!(
        request.query,
        query
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>()
    );
    assert_eq!(request.body, body);
    assert_eq!(
        request.form,
        form.into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>()
    );
    assert_eq!(request.token.as_deref(), token);
    assert_eq!(request.basic_auth.as_deref(), basic_auth);
}

#[test]
fn api_request_url_trims_base_url_and_encodes_query_pairs() {
    let request = ApiRequest {
        method: HttpMethod::Get,
        base_url: "https://api.ticktick.com/".into(),
        path: "/open/v1/focus".into(),
        query: vec![
            ("from".into(), "2026-04-01 00:00".into()),
            ("scope".into(), "tasks:read tasks:write".into()),
        ],
        body: None,
        form: Vec::new(),
        token: None,
        basic_auth: None,
    };

    let url = request.url().unwrap();
    assert_eq!(
        url.as_str(),
        "https://api.ticktick.com/open/v1/focus?from=2026-04-01+00%3A00&scope=tasks%3Aread+tasks%3Awrite"
    );
}

#[test]
fn task_endpoints_build_expected_requests() {
    let client = ApiClient::new("https://api.ticktick.com");
    let create_body = json!({"title": "Task", "projectId": "proj-1"});
    let update_body = json!({"id": "task-1", "projectId": "proj-1", "title": "Updated"});
    let move_body = json!({"fromProjectId": "proj-1", "toProjectId": "proj-2", "taskId": "task-1"});
    let completed_body = json!({
        "projectIds": ["proj-1"],
        "startDate": "2026-03-01T00:58:20.000+0000",
        "endDate": "2026-03-05T10:58:20.000+0000"
    });
    let filter_body = json!({"projectIds": ["proj-1"], "tag": ["urgent"], "status": [0]});

    assert_request(
        client
            .get_task("proj-1", "task-1", Some("tok".into()))
            .unwrap(),
        HttpMethod::Get,
        "/open/v1/project/proj-1/task/task-1",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .create_task(create_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/task",
        vec![],
        Some(create_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .update_task("task-1", update_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/task/task-1",
        vec![],
        Some(update_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .complete_task("proj-1", "task-1", Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/project/proj-1/task/task-1/complete",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .delete_task("proj-1", "task-1", Some("tok".into()))
            .unwrap(),
        HttpMethod::Delete,
        "/open/v1/project/proj-1/task/task-1",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .move_task(move_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/task/move",
        vec![],
        Some(move_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .completed_tasks(completed_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/task/completed",
        vec![],
        Some(completed_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .filter_tasks(filter_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/task/filter",
        vec![],
        Some(filter_body),
        vec![],
        Some("tok"),
        None,
    );
}

#[test]
fn project_endpoints_build_expected_requests() {
    let client = ApiClient::new("https://api.ticktick.com");
    let create_body = json!({"name": "Inbox"});
    let update_body = json!({"name": "Renamed"});

    assert_request(
        client.list_projects(Some("tok".into())).unwrap(),
        HttpMethod::Get,
        "/open/v1/project",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client.get_project("proj-1", Some("tok".into())).unwrap(),
        HttpMethod::Get,
        "/open/v1/project/proj-1",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .get_project_data("proj-1", Some("tok".into()))
            .unwrap(),
        HttpMethod::Get,
        "/open/v1/project/proj-1/data",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .create_project(create_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/project",
        vec![],
        Some(create_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .update_project("proj-1", update_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/project/proj-1",
        vec![],
        Some(update_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client.delete_project("proj-1", Some("tok".into())).unwrap(),
        HttpMethod::Delete,
        "/open/v1/project/proj-1",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
}

#[test]
fn focus_and_habit_endpoints_build_expected_requests() {
    let client = ApiClient::new("https://api.ticktick.com");
    let habit_body = json!({"name": "Walk"});
    let checkin_body = json!({"stamp": 20260422, "value": 1.0, "goal": 1.0});

    assert_request(
        client.get_focus("focus-1", 1, Some("tok".into())).unwrap(),
        HttpMethod::Get,
        "/open/v1/focus/focus-1",
        vec![("type", "1")],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .list_focuses(
                "2026-04-01T00:00:00+0800",
                "2026-04-30T00:00:00+0800",
                0,
                Some("tok".into()),
            )
            .unwrap(),
        HttpMethod::Get,
        "/open/v1/focus",
        vec![
            ("from", "2026-04-01T00:00:00+0800"),
            ("to", "2026-04-30T00:00:00+0800"),
            ("type", "0"),
        ],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .delete_focus("focus-1", 0, Some("tok".into()))
            .unwrap(),
        HttpMethod::Delete,
        "/open/v1/focus/focus-1",
        vec![("type", "0")],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client.get_habit("habit-1", Some("tok".into())).unwrap(),
        HttpMethod::Get,
        "/open/v1/habit/habit-1",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client.list_habits(Some("tok".into())).unwrap(),
        HttpMethod::Get,
        "/open/v1/habit",
        vec![],
        None,
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .create_habit(habit_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/habit",
        vec![],
        Some(habit_body.clone()),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .update_habit("habit-1", habit_body, Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/habit/habit-1",
        vec![],
        Some(json!({"name": "Walk"})),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .check_in_habit("habit-1", checkin_body.clone(), Some("tok".into()))
            .unwrap(),
        HttpMethod::Post,
        "/open/v1/habit/habit-1/checkin",
        vec![],
        Some(checkin_body),
        vec![],
        Some("tok"),
        None,
    );
    assert_request(
        client
            .habit_checkins("habit-1,habit-2", 20260401, 20260407, Some("tok".into()))
            .unwrap(),
        HttpMethod::Get,
        "/open/v1/habit/checkins",
        vec![
            ("habitIds", "habit-1,habit-2"),
            ("from", "20260401"),
            ("to", "20260407"),
        ],
        None,
        vec![],
        Some("tok"),
        None,
    );
}

#[test]
fn oauth_exchange_request_builds_form_and_basic_auth() {
    let request = ApiClient::new("")
        .oauth_exchange_code(
            "client-id",
            "client-secret",
            "code-123",
            "tasks:read tasks:write",
            "https://localhost/callback",
        )
        .unwrap();

    assert_request(
        request,
        HttpMethod::Post,
        "/oauth/token",
        vec![],
        None,
        vec![
            ("code", "code-123"),
            ("grant_type", "authorization_code"),
            ("scope", "tasks:read tasks:write"),
            ("redirect_uri", "https://localhost/callback"),
        ],
        None,
        Some("client-id:client-secret"),
    );
}
