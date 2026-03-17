mod cli;
mod client;
mod commands;
mod error;
mod queries;
mod schema;

use clap::Parser;
use serde_json::json;

use cli::{AuthCommand, Cli, Command};
use client::PrefixClient;
use error::{ErrorResponse, PfxError};

fn output_success(data: serde_json::Value) {
    let envelope = json!({ "ok": true, "data": data });
    println!("{}", serde_json::to_string(&envelope).unwrap());
}

fn output_error(err: &PfxError) {
    let error_resp = ErrorResponse::from(err);
    let envelope = json!({ "ok": false, "error": error_resp });
    println!("{}", serde_json::to_string(&envelope).unwrap());
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let result = run(cli).await;

    match result {
        Ok(data) => {
            output_success(data);
            std::process::exit(0);
        }
        Err(err) => {
            output_error(&err);
            std::process::exit(1);
        }
    }
}

async fn run(cli: Cli) -> Result<serde_json::Value, PfxError> {
    match &cli.command {
        Command::Describe { command_path } => {
            let cmd = <Cli as clap::CommandFactory>::command();
            Ok(commands::describe::describe_commands(&cmd, command_path))
        }

        Command::Auth { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            match command {
                AuthCommand::Whoami => commands::auth::handle_whoami(&client).await,
                AuthCommand::ApiKey { command: api_cmd } => {
                    commands::auth::handle_api_key(&client, api_cmd).await
                }
            }
        }

        Command::Channel { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            commands::channel::handle(&client, command).await
        }

        Command::Package { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            commands::package::handle(&client, command).await
        }
    }
}
