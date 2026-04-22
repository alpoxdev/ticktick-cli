use std::{
    fs,
    io::{self, Read},
};

use clap::{Args, Parser, Subcommand};
use serde_json::Value;

use crate::{
    api::{ApiClient, ApiRequest},
    config::{API_BASE_URL, AuthUrlConfig, OAUTH_BASE_URL},
    error::{Result, TickTickError},
};

#[derive(Debug, Parser)]
#[command(name = "ticktick", version, about = "TickTick Open API CLI")]
pub struct Cli {
    #[arg(long, env = "TICKTICK_ACCESS_TOKEN", global = true)]
    pub token: Option<String>,

    #[arg(long, env = "TICKTICK_CLIENT_ID", global = true)]
    pub client_id: Option<String>,

    #[arg(long, env = "TICKTICK_CLIENT_SECRET", global = true)]
    pub client_secret: Option<String>,

    #[arg(long, global = true, default_value = API_BASE_URL)]
    pub base_url: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(name = "oauth")]
    OAuth {
        #[command(subcommand)]
        command: OAuthCommand,
    },
    Task {
        #[command(subcommand)]
        command: TaskCommand,
    },
    Project {
        #[command(subcommand)]
        command: ProjectCommand,
    },
    Focus {
        #[command(subcommand)]
        command: FocusCommand,
    },
    Habit {
        #[command(subcommand)]
        command: HabitCommand,
    },
}

#[derive(Debug)]
pub enum Action {
    Api(ApiRequest),
    PrintJson(Value),
}

impl Cli {
    pub fn into_action(self) -> Result<Action> {
        let base_url = self.base_url;
        let token = self.token;
        let client_id = self.client_id;
        let client_secret = self.client_secret;

        match self.command {
            Command::OAuth { command } => Self::oauth_action(client_id, client_secret, command),
            Command::Task { command } => Self::task_action(base_url, token, command),
            Command::Project { command } => Self::project_action(base_url, token, command),
            Command::Focus { command } => Self::focus_action(base_url, token, command),
            Command::Habit { command } => Self::habit_action(base_url, token, command),
        }
    }

    pub fn into_request(self, default_token: Option<String>) -> Result<ApiRequest> {
        match self.into_action()? {
            Action::Api(mut request) => {
                if request.token.is_none() {
                    request.token = default_token;
                }
                if request.token.is_none() && request.basic_auth.is_none() {
                    return Err(TickTickError::MissingAccessToken);
                }
                Ok(request)
            }
            Action::PrintJson(_) => Err(TickTickError::MissingJsonInput),
        }
    }

    fn api_client(base_url: String) -> ApiClient {
        ApiClient::new(base_url)
    }

    fn require_client_id(client_id: Option<String>) -> Result<String> {
        client_id.ok_or(TickTickError::MissingClientId)
    }

    fn require_client_secret(client_secret: Option<String>) -> Result<String> {
        client_secret.ok_or(TickTickError::MissingClientSecret)
    }

    fn oauth_action(
        client_id: Option<String>,
        client_secret: Option<String>,
        command: OAuthCommand,
    ) -> Result<Action> {
        match command {
            OAuthCommand::Authorize {
                scope,
                state,
                redirect_uri,
            } => {
                let cfg = AuthUrlConfig {
                    client_id: Self::require_client_id(client_id)?,
                    scope,
                    state,
                    redirect_uri,
                };
                Ok(Action::PrintJson(
                    serde_json::json!({ "authorize_url": cfg.build_url()?.to_string() }),
                ))
            }
            OAuthCommand::Exchange {
                code,
                scope,
                redirect_uri,
            } => {
                let client = ApiClient::new(OAUTH_BASE_URL);
                Ok(Action::Api(client.oauth_exchange_code(
                    &Self::require_client_id(client_id)?,
                    &Self::require_client_secret(client_secret)?,
                    &code,
                    &scope,
                    &redirect_uri,
                )?))
            }
        }
    }

    fn task_action(
        base_url: String,
        token: Option<String>,
        command: TaskCommand,
    ) -> Result<Action> {
        let client = Self::api_client(base_url);
        let request = match command {
            TaskCommand::Get {
                project_id,
                task_id,
            } => client.get_task(&project_id, &task_id, token)?,
            TaskCommand::Create { input } => {
                client.create_task(input.read_required_json()?, token)?
            }
            TaskCommand::Update { task_id, input } => {
                client.update_task(&task_id, input.read_required_json()?, token)?
            }
            TaskCommand::Complete {
                project_id,
                task_id,
            } => client.complete_task(&project_id, &task_id, token)?,
            TaskCommand::Delete {
                project_id,
                task_id,
            } => client.delete_task(&project_id, &task_id, token)?,
            TaskCommand::Move { input } => client.move_task(input.read_required_json()?, token)?,
            TaskCommand::Completed { input } => {
                client.completed_tasks(input.read_required_json()?, token)?
            }
            TaskCommand::Filter { input } => {
                client.filter_tasks(input.read_required_json()?, token)?
            }
        };
        Ok(Action::Api(request))
    }

    fn project_action(
        base_url: String,
        token: Option<String>,
        command: ProjectCommand,
    ) -> Result<Action> {
        let client = Self::api_client(base_url);
        let request = match command {
            ProjectCommand::List => client.list_projects(token)?,
            ProjectCommand::Get { project_id } => client.get_project(&project_id, token)?,
            ProjectCommand::Data { project_id } => client.get_project_data(&project_id, token)?,
            ProjectCommand::Create { input } => {
                client.create_project(input.read_required_json()?, token)?
            }
            ProjectCommand::Update { project_id, input } => {
                client.update_project(&project_id, input.read_required_json()?, token)?
            }
            ProjectCommand::Delete { project_id } => client.delete_project(&project_id, token)?,
        };
        Ok(Action::Api(request))
    }

    fn focus_action(
        base_url: String,
        token: Option<String>,
        command: FocusCommand,
    ) -> Result<Action> {
        let client = Self::api_client(base_url);
        let request = match command {
            FocusCommand::Get {
                focus_id,
                focus_type,
            } => client.get_focus(&focus_id, focus_type, token)?,
            FocusCommand::List {
                from,
                to,
                focus_type,
            } => client.list_focuses(&from, &to, focus_type, token)?,
            FocusCommand::Delete {
                focus_id,
                focus_type,
            } => client.delete_focus(&focus_id, focus_type, token)?,
        };
        Ok(Action::Api(request))
    }

    fn habit_action(
        base_url: String,
        token: Option<String>,
        command: HabitCommand,
    ) -> Result<Action> {
        let client = Self::api_client(base_url);
        let request = match command {
            HabitCommand::Get { habit_id } => client.get_habit(&habit_id, token)?,
            HabitCommand::List => client.list_habits(token)?,
            HabitCommand::Create { input } => {
                client.create_habit(input.read_required_json()?, token)?
            }
            HabitCommand::Update { habit_id, input } => {
                client.update_habit(&habit_id, input.read_required_json()?, token)?
            }
            HabitCommand::Checkin { habit_id, input } => {
                client.check_in_habit(&habit_id, input.read_required_json()?, token)?
            }
            HabitCommand::Checkins {
                habit_ids,
                from,
                to,
            } => client.habit_checkins(&habit_ids, from, to, token)?,
        };
        Ok(Action::Api(request))
    }
}

#[derive(Debug, Subcommand)]
pub enum OAuthCommand {
    /// Build the TickTick OAuth authorization URL.
    Authorize {
        #[arg(long, default_value = "tasks:read tasks:write")]
        scope: String,
        #[arg(long)]
        state: String,
        #[arg(long)]
        redirect_uri: String,
    },
    /// Exchange an OAuth authorization code for an access token.
    Exchange {
        #[arg(long)]
        code: String,
        #[arg(long, default_value = "tasks:read tasks:write")]
        scope: String,
        #[arg(long)]
        redirect_uri: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum TaskCommand {
    Get {
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        task_id: String,
    },
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    Update {
        #[arg(long)]
        task_id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    Complete {
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        task_id: String,
    },
    Delete {
        #[arg(long)]
        project_id: String,
        #[arg(long)]
        task_id: String,
    },
    Move {
        #[command(flatten)]
        input: JsonInput,
    },
    Completed {
        #[command(flatten)]
        input: JsonInput,
    },
    Filter {
        #[command(flatten)]
        input: JsonInput,
    },
}

#[derive(Debug, Subcommand)]
pub enum ProjectCommand {
    List,
    Get {
        #[arg(long)]
        project_id: String,
    },
    Data {
        #[arg(long)]
        project_id: String,
    },
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    Update {
        #[arg(long)]
        project_id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    Delete {
        #[arg(long)]
        project_id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum FocusCommand {
    Get {
        #[arg(long)]
        focus_id: String,
        #[arg(long = "type")]
        focus_type: i32,
    },
    List {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long = "type")]
        focus_type: i32,
    },
    Delete {
        #[arg(long)]
        focus_id: String,
        #[arg(long = "type")]
        focus_type: i32,
    },
}

#[derive(Debug, Subcommand)]
pub enum HabitCommand {
    Get {
        #[arg(long)]
        habit_id: String,
    },
    List,
    Create {
        #[command(flatten)]
        input: JsonInput,
    },
    Update {
        #[arg(long)]
        habit_id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    Checkin {
        #[arg(long)]
        habit_id: String,
        #[command(flatten)]
        input: JsonInput,
    },
    Checkins {
        #[arg(long)]
        habit_ids: String,
        #[arg(long)]
        from: i32,
        #[arg(long)]
        to: i32,
    },
}

#[derive(Debug, Clone, Args)]
pub struct JsonInput {
    #[arg(long)]
    pub json: Option<String>,
    #[arg(long)]
    pub json_file: Option<String>,
    #[arg(long)]
    pub json_stdin: bool,
}

impl JsonInput {
    pub fn read_required_json(&self) -> Result<Value> {
        self.read_json()?.ok_or(TickTickError::MissingJsonInput)
    }

    fn read_json(&self) -> Result<Option<Value>> {
        let count =
            self.json.is_some() as u8 + self.json_file.is_some() as u8 + self.json_stdin as u8;
        if count > 1 {
            return Err(TickTickError::ConflictingJsonInput);
        }
        if let Some(raw) = &self.json {
            return Ok(Some(serde_json::from_str(raw)?));
        }
        if let Some(path) = &self.json_file {
            let content =
                fs::read_to_string(path).map_err(|source| TickTickError::ReadJsonFile {
                    path: path.clone(),
                    source,
                })?;
            return Ok(Some(serde_json::from_str(&content)?));
        }
        if self.json_stdin {
            let mut content = String::new();
            io::stdin()
                .read_to_string(&mut content)
                .map_err(TickTickError::ReadStdin)?;
            return Ok(Some(serde_json::from_str(&content)?));
        }
        Ok(None)
    }
}
