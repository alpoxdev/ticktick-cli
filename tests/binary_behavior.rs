use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn binary_rejects_authenticated_commands_without_token_before_network_call() {
    let mut cmd = Command::cargo_bin("ticktick-cli").unwrap();
    cmd.args(["project", "list"]);

    cmd.assert().failure().stderr(predicate::str::contains(
        "missing access token: pass --token or set TICKTICK_ACCESS_TOKEN",
    ));
}
