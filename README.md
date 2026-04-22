# ticktick-cli

A Rust CLI for the official TickTick Open API, implemented from the local official docs in `docs/ticktick-openapi.md`.

## Features

- Covers every documented Open API endpoint in the local official docs.
- OAuth helpers:
  - build the authorization URL
  - exchange authorization code for access token
- Uses `--token` or `TICKTICK_ACCESS_TOKEN` for authenticated API requests.
- Uses `--client-id` / `TICKTICK_CLIENT_ID` and `--client-secret` / `TICKTICK_CLIENT_SECRET` for OAuth.
- Pretty-prints API responses as JSON.
- Supports complex request bodies with:
  - `--json '...json...'`
  - `--json-file payload.json`
  - `--json-stdin`

## Build

```bash
cargo build
```

## Environment variables

```bash
export TICKTICK_ACCESS_TOKEN="..."
export TICKTICK_CLIENT_ID="..."
export TICKTICK_CLIENT_SECRET="..."
```

## OAuth

### Build authorization URL

Maps to the docs' OAuth authorize step.

```bash
cargo run -- oauth authorize \
  --client-id "$TICKTICK_CLIENT_ID" \
  --redirect-uri "http://localhost:8080/callback" \
  --scope "tasks:read tasks:write" \
  --state "xyz"
```

### Exchange authorization code for access token

Maps to `POST https://ticktick.com/oauth/token`.

```bash
cargo run -- oauth exchange \
  --client-id "$TICKTICK_CLIENT_ID" \
  --client-secret "$TICKTICK_CLIENT_SECRET" \
  --code "AUTH_CODE" \
  --redirect-uri "http://localhost:8080/callback" \
  --scope "tasks:read tasks:write"
```

## Endpoint-to-command mapping

### Task

#### Get task by project ID and task ID
`GET /open/v1/project/{projectId}/task/{taskId}`

```bash
cargo run -- task get --project-id PROJECT_ID --task-id TASK_ID
```

#### Create task
`POST /open/v1/task`

```bash
cargo run -- task create --json '{
  "title": "Task Title",
  "projectId": "PROJECT_ID"
}'
```

#### Update task
`POST /open/v1/task/{taskId}`

```bash
cargo run -- task update --task-id TASK_ID --json '{
  "id": "TASK_ID",
  "projectId": "PROJECT_ID",
  "title": "Updated title"
}'
```

#### Complete task
`POST /open/v1/project/{projectId}/task/{taskId}/complete`

```bash
cargo run -- task complete --project-id PROJECT_ID --task-id TASK_ID
```

#### Delete task
`DELETE /open/v1/project/{projectId}/task/{taskId}`

```bash
cargo run -- task delete --project-id PROJECT_ID --task-id TASK_ID
```

#### Move task
`POST /open/v1/task/move`

```bash
cargo run -- task move --json '[
  {
    "fromProjectId": "FROM_PROJECT",
    "toProjectId": "TO_PROJECT",
    "taskId": "TASK_ID"
  }
]'
```

#### List completed tasks
`POST /open/v1/task/completed`

```bash
cargo run -- task completed --json '{
  "projectIds": ["PROJECT_ID"],
  "startDate": "2026-03-01T00:58:20.000+0000",
  "endDate": "2026-03-05T10:58:20.000+0000"
}'
```

#### Filter tasks
`POST /open/v1/task/filter`

```bash
cargo run -- task filter --json '{
  "projectIds": ["PROJECT_ID"],
  "startDate": "2026-03-01T00:58:20.000+0000",
  "endDate": "2026-03-06T10:58:20.000+0000",
  "priority": [0],
  "tag": ["urgent"],
  "status": [0]
}'
```

### Project

#### Get user projects
`GET /open/v1/project`

```bash
cargo run -- project list
```

#### Get project by ID
`GET /open/v1/project/{projectId}`

```bash
cargo run -- project get --project-id PROJECT_ID
```

#### Get project with data
`GET /open/v1/project/{projectId}/data`

```bash
cargo run -- project data --project-id PROJECT_ID
```

#### Create project
`POST /open/v1/project`

```bash
cargo run -- project create --json '{
  "name": "project name",
  "color": "#F18181",
  "viewMode": "list",
  "kind": "TASK"
}'
```

#### Update project
`POST /open/v1/project/{projectId}`

```bash
cargo run -- project update --project-id PROJECT_ID --json '{
  "name": "Project Name",
  "color": "#F18181",
  "viewMode": "list",
  "kind": "TASK"
}'
```

#### Delete project
`DELETE /open/v1/project/{projectId}`

```bash
cargo run -- project delete --project-id PROJECT_ID
```

### Focus

#### Get focus by ID
`GET /open/v1/focus/{focusId}?type=...`

```bash
cargo run -- focus get --focus-id FOCUS_ID --type 0
```

#### Get focuses by time range
`GET /open/v1/focus?from=...&to=...&type=...`

```bash
cargo run -- focus list \
  --from "2026-04-01T00:00:00+0800" \
  --to "2026-04-02T00:00:00+0800" \
  --type 1
```

#### Delete focus
`DELETE /open/v1/focus/{focusId}?type=...`

```bash
cargo run -- focus delete --focus-id FOCUS_ID --type 0
```

### Habit

#### Get habit by ID
`GET /open/v1/habit/{habitId}`

```bash
cargo run -- habit get --habit-id HABIT_ID
```

#### Get all habits
`GET /open/v1/habit`

```bash
cargo run -- habit list
```

#### Create habit
`POST /open/v1/habit`

```bash
cargo run -- habit create --json '{
  "name": "Read",
  "iconRes": "habit_reading",
  "color": "#4D8CF5",
  "type": "Boolean",
  "goal": 1.0,
  "step": 1.0,
  "unit": "Count",
  "repeatRule": "RRULE:FREQ=DAILY;INTERVAL=1",
  "recordEnable": false
}'
```

#### Update habit
`POST /open/v1/habit/{habitId}`

```bash
cargo run -- habit update --habit-id HABIT_ID --json '{
  "name": "Read more",
  "goal": 2.0,
  "repeatRule": "RRULE:FREQ=DAILY;INTERVAL=1"
}'
```

#### Create or update habit check-in
`POST /open/v1/habit/{habitId}/checkin`

```bash
cargo run -- habit checkin --habit-id HABIT_ID --json '{
  "stamp": 20260407,
  "value": 1.0,
  "goal": 1.0
}'
```

#### Get habit check-ins
`GET /open/v1/habit/checkins?habitIds=...&from=...&to=...`

```bash
cargo run -- habit checkins \
  --habit-ids "habit-1,habit-2" \
  --from 20260401 \
  --to 20260407
```

## JSON input from file or stdin

From file:

```bash
cargo run -- task create --json-file payload.json
```

From stdin:

```bash
echo '{"title":"Task Title","projectId":"PROJECT_ID"}' | \
  cargo run -- task create --json-stdin
```

## Notes

- The default API base URL is `https://api.ticktick.com`.
- OAuth token exchange always targets `https://ticktick.com/oauth/token`, per the official docs.
- For authenticated API commands, pass `--token` or set `TICKTICK_ACCESS_TOKEN`.
