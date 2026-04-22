pub mod api;
pub mod cli;
pub mod config;
pub mod error;
pub mod output;

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        api::{ApiClient, HttpMethod},
        cli::Cli,
        config::AuthUrlConfig,
    };
    use clap::Parser;
    use serde_json::json;

    #[test]
    fn builds_authorize_url_from_docs() {
        let cfg = AuthUrlConfig {
            client_id: "client_id".into(),
            scope: "tasks:read tasks:write".into(),
            state: "state123".into(),
            redirect_uri: "https://localhost/callback".into(),
        };

        let url = cfg.build_url().unwrap();
        assert_eq!(
            url.as_str(),
            "https://ticktick.com/oauth/authorize?client_id=client_id&scope=tasks%3Aread+tasks%3Awrite&state=state123&redirect_uri=https%3A%2F%2Flocalhost%2Fcallback&response_type=code"
        );
    }

    #[test]
    fn oauth_authorize_requires_state_per_docs() {
        let result = Cli::try_parse_from([
            "ticktick",
            "oauth",
            "authorize",
            "--client-id",
            "client_id",
            "--redirect-uri",
            "https://localhost/callback",
        ]);

        assert!(result.is_err());
        let err = result.unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.contains("--state"),
            "unexpected clap error: {rendered}"
        );
    }

    #[test]
    fn parses_habit_checkins_cli_args() {
        let cli = Cli::parse_from([
            "ticktick",
            "habit",
            "checkins",
            "--habit-ids",
            "h1,h2",
            "--from",
            "20260401",
            "--to",
            "20260407",
            "--token",
            "tok",
        ]);

        let request = cli.into_request(None).unwrap();
        assert_eq!(request.method, HttpMethod::Get);
        assert_eq!(request.path, "/open/v1/habit/checkins");
        assert_eq!(
            request.query,
            vec![
                ("habitIds".into(), "h1,h2".into()),
                ("from".into(), "20260401".into()),
                ("to".into(), "20260407".into()),
            ]
        );
        assert_eq!(request.token.as_deref(), Some("tok"));
    }

    #[test]
    fn builds_filter_tasks_request_with_json() {
        let cli = Cli::parse_from([
            "ticktick",
            "task",
            "filter",
            "--token",
            "tok",
            "--json",
            r#"{"projectIds":["p1"],"priority":[0],"tag":["urgent"],"status":[0]}"#,
        ]);

        let request = cli.into_request(None).unwrap();
        assert_eq!(request.method, HttpMethod::Post);
        assert_eq!(request.path, "/open/v1/task/filter");
        assert_eq!(
            request.body,
            Some(json!({
                "projectIds": ["p1"],
                "priority": [0],
                "tag": ["urgent"],
                "status": [0]
            }))
        );
    }

    #[test]
    fn builds_focus_delete_request_with_required_query() {
        let request = ApiClient::new("https://api.ticktick.com")
            .delete_focus("focus-1", 0, Some("tok".into()))
            .unwrap();

        assert_eq!(request.method, HttpMethod::Delete);
        assert_eq!(request.path, "/open/v1/focus/focus-1");
        assert_eq!(request.query, vec![("type".into(), "0".into())]);
    }

    #[test]
    fn builds_oauth_token_exchange_request() {
        let request = ApiClient::new("https://ticktick.com")
            .oauth_exchange_code(
                "client-id",
                "client-secret",
                "code-123",
                "tasks:read tasks:write",
                "https://localhost/callback",
            )
            .unwrap();

        assert_eq!(request.method, HttpMethod::Post);
        assert_eq!(request.path, "/oauth/token");
        assert_eq!(
            request.basic_auth.as_deref(),
            Some("client-id:client-secret")
        );
        assert_eq!(
            request.form,
            vec![
                ("code".into(), "code-123".into()),
                ("grant_type".into(), "authorization_code".into()),
                ("scope".into(), "tasks:read tasks:write".into()),
                ("redirect_uri".into(), "https://localhost/callback".into()),
            ]
        );
    }
}
