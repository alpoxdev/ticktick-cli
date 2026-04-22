use clap::Parser;
use ticktick_cli::{
    api::ApiClient,
    cli::{Action, Cli},
    error::TickTickError,
    output::print_json,
};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> ticktick_cli::error::Result<()> {
    let cli = Cli::parse();
    let client = ApiClient::new(cli.base_url.clone());
    match cli.into_action()? {
        Action::Api(request) => {
            if request.token.is_none() && request.basic_auth.is_none() {
                return Err(TickTickError::MissingAccessToken);
            }
            let response = client.execute(request).await?;
            print_json(&response)?;
        }
        Action::PrintJson(value) => print_json(&value)?,
    }
    Ok(())
}
