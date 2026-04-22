use base64::{Engine as _, engine::general_purpose::STANDARD};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use url::Url;

use crate::{
    config::API_BASE_URL,
    error::{Result, TickTickError},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Delete,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiRequest {
    pub method: HttpMethod,
    pub base_url: String,
    pub path: String,
    pub query: Vec<(String, String)>,
    pub body: Option<Value>,
    pub form: Vec<(String, String)>,
    pub token: Option<String>,
    pub basic_auth: Option<String>,
}

impl ApiRequest {
    pub fn url(&self) -> Result<Url> {
        let mut url = Url::parse(&format!(
            "{}{}",
            self.base_url.trim_end_matches('/'),
            self.path
        ))?;
        if !self.query.is_empty() {
            let mut pairs = url.query_pairs_mut();
            for (k, v) in &self.query {
                pairs.append_pair(k, v);
            }
        }
        Ok(url)
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    base_url: String,
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new(API_BASE_URL)
    }
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    fn request(
        &self,
        method: HttpMethod,
        path: impl Into<String>,
        token: Option<String>,
    ) -> ApiRequest {
        ApiRequest {
            method,
            base_url: self.base_url.clone(),
            path: path.into(),
            query: Vec::new(),
            body: None,
            form: Vec::new(),
            token,
            basic_auth: None,
        }
    }

    pub fn get_task(
        &self,
        project_id: &str,
        task_id: &str,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        Ok(self.request(
            HttpMethod::Get,
            format!("/open/v1/project/{project_id}/task/{task_id}"),
            token,
        ))
    }

    pub fn create_task(&self, body: Value, token: Option<String>) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, "/open/v1/task", token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn update_task(
        &self,
        task_id: &str,
        body: Value,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, format!("/open/v1/task/{task_id}"), token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn complete_task(
        &self,
        project_id: &str,
        task_id: &str,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        Ok(self.request(
            HttpMethod::Post,
            format!("/open/v1/project/{project_id}/task/{task_id}/complete"),
            token,
        ))
    }

    pub fn delete_task(
        &self,
        project_id: &str,
        task_id: &str,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        Ok(self.request(
            HttpMethod::Delete,
            format!("/open/v1/project/{project_id}/task/{task_id}"),
            token,
        ))
    }

    pub fn move_task(&self, body: Value, token: Option<String>) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, "/open/v1/task/move", token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn completed_tasks(&self, body: Value, token: Option<String>) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, "/open/v1/task/completed", token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn filter_tasks(&self, body: Value, token: Option<String>) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, "/open/v1/task/filter", token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn list_projects(&self, token: Option<String>) -> Result<ApiRequest> {
        Ok(self.request(HttpMethod::Get, "/open/v1/project", token))
    }

    pub fn get_project(&self, project_id: &str, token: Option<String>) -> Result<ApiRequest> {
        Ok(self.request(
            HttpMethod::Get,
            format!("/open/v1/project/{project_id}"),
            token,
        ))
    }

    pub fn get_project_data(&self, project_id: &str, token: Option<String>) -> Result<ApiRequest> {
        Ok(self.request(
            HttpMethod::Get,
            format!("/open/v1/project/{project_id}/data"),
            token,
        ))
    }

    pub fn create_project(&self, body: Value, token: Option<String>) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, "/open/v1/project", token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn update_project(
        &self,
        project_id: &str,
        body: Value,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(
            HttpMethod::Post,
            format!("/open/v1/project/{project_id}"),
            token,
        );
        req.body = Some(body);
        Ok(req)
    }

    pub fn delete_project(&self, project_id: &str, token: Option<String>) -> Result<ApiRequest> {
        Ok(self.request(
            HttpMethod::Delete,
            format!("/open/v1/project/{project_id}"),
            token,
        ))
    }

    pub fn get_focus(
        &self,
        focus_id: &str,
        focus_type: i32,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Get, format!("/open/v1/focus/{focus_id}"), token);
        req.query.push(("type".into(), focus_type.to_string()));
        Ok(req)
    }

    pub fn list_focuses(
        &self,
        from: &str,
        to: &str,
        focus_type: i32,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Get, "/open/v1/focus", token);
        req.query = vec![
            ("from".into(), from.into()),
            ("to".into(), to.into()),
            ("type".into(), focus_type.to_string()),
        ];
        Ok(req)
    }

    pub fn delete_focus(
        &self,
        focus_id: &str,
        focus_type: i32,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(
            HttpMethod::Delete,
            format!("/open/v1/focus/{focus_id}"),
            token,
        );
        req.query.push(("type".into(), focus_type.to_string()));
        Ok(req)
    }

    pub fn get_habit(&self, habit_id: &str, token: Option<String>) -> Result<ApiRequest> {
        Ok(self.request(HttpMethod::Get, format!("/open/v1/habit/{habit_id}"), token))
    }

    pub fn list_habits(&self, token: Option<String>) -> Result<ApiRequest> {
        Ok(self.request(HttpMethod::Get, "/open/v1/habit", token))
    }

    pub fn create_habit(&self, body: Value, token: Option<String>) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Post, "/open/v1/habit", token);
        req.body = Some(body);
        Ok(req)
    }

    pub fn update_habit(
        &self,
        habit_id: &str,
        body: Value,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(
            HttpMethod::Post,
            format!("/open/v1/habit/{habit_id}"),
            token,
        );
        req.body = Some(body);
        Ok(req)
    }

    pub fn check_in_habit(
        &self,
        habit_id: &str,
        body: Value,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(
            HttpMethod::Post,
            format!("/open/v1/habit/{habit_id}/checkin"),
            token,
        );
        req.body = Some(body);
        Ok(req)
    }

    pub fn habit_checkins(
        &self,
        habit_ids: &str,
        from: i32,
        to: i32,
        token: Option<String>,
    ) -> Result<ApiRequest> {
        let mut req = self.request(HttpMethod::Get, "/open/v1/habit/checkins", token);
        req.query = vec![
            ("habitIds".into(), habit_ids.into()),
            ("from".into(), from.to_string()),
            ("to".into(), to.to_string()),
        ];
        Ok(req)
    }

    pub fn oauth_exchange_code(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        scope: &str,
        redirect_uri: &str,
    ) -> Result<ApiRequest> {
        let mut req = ApiRequest {
            method: HttpMethod::Post,
            base_url: self.base_url.clone(),
            path: "/oauth/token".into(),
            query: Vec::new(),
            body: None,
            form: vec![
                ("code".into(), code.into()),
                ("grant_type".into(), "authorization_code".into()),
                ("scope".into(), scope.into()),
                ("redirect_uri".into(), redirect_uri.into()),
            ],
            token: None,
            basic_auth: Some(format!("{client_id}:{client_secret}")),
        };
        if req.base_url.is_empty() {
            req.base_url = API_BASE_URL.into();
        }
        Ok(req)
    }

    pub async fn execute(&self, request: ApiRequest) -> Result<Value> {
        let http = reqwest::Client::new();
        let url = request.url()?;
        let mut builder = match request.method {
            HttpMethod::Get => http.get(url),
            HttpMethod::Post => http.post(url),
            HttpMethod::Delete => http.delete(url),
        };

        builder = builder.header(ACCEPT, "application/json");

        if let Some(token) = &request.token {
            builder = builder.header(AUTHORIZATION, format!("Bearer {token}"));
        }
        if let Some(creds) = &request.basic_auth {
            builder = builder.header(AUTHORIZATION, format!("Basic {}", STANDARD.encode(creds)));
        }
        if let Some(body) = &request.body {
            builder = builder.header(CONTENT_TYPE, "application/json").json(body);
        }
        if !request.form.is_empty() {
            builder = builder
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .form(&request.form);
        }

        let response = builder.send().await?;
        let status = response.status();
        let text = response.text().await?;
        if !status.is_success() {
            return Err(TickTickError::ApiStatus { status, body: text });
        }
        if text.trim().is_empty() {
            return Ok(Value::Null);
        }
        match serde_json::from_str::<Value>(&text) {
            Ok(value) => Ok(value),
            Err(_) => Ok(Value::String(text)),
        }
    }
}
