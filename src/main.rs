mod cli;
mod client;
mod commands;
mod error;
mod format;
mod queries;
mod schema;

use clap::Parser;

use cli::{ApiKeyCommand, AuthCommand, ChannelCommand, Cli, Command, PackageCommand};
use client::PrefixClient;
use error::PfxError;
use format::{CommandOutput, OutputKind};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let json = cli.json;

    let result = run(&cli).await;

    match result {
        Ok(output) => {
            if json {
                println!("{}", format::format_json(&output));
            } else {
                print!("{}", format::format_human(&output));
            }
            std::process::exit(0);
        }
        Err(err) => {
            if json {
                println!("{}", format::format_json_error(&err));
            } else {
                eprintln!("{}", format::format_human_error(&err));
            }
            std::process::exit(1);
        }
    }
}

async fn run(cli: &Cli) -> Result<CommandOutput, PfxError> {
    match &cli.command {
        Command::Describe { command_path } => {
            let cmd = <Cli as clap::CommandFactory>::command();
            Ok(CommandOutput::raw(commands::describe::describe_commands(
                &cmd,
                command_path,
            )))
        }

        Command::Job { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            let val = commands::package::handle_job(&client, command).await?;
            Ok(CommandOutput::new(val, OutputKind::BackgroundJob))
        }

        Command::Auth { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            match command {
                AuthCommand::Status => {
                    let val = client.authentication_status().await?;
                    Ok(CommandOutput::raw(val))
                }
                AuthCommand::Whoami => {
                    let val = commands::auth::handle_whoami(&client).await?;
                    Ok(CommandOutput::new(val, OutputKind::User))
                }
                AuthCommand::ApiKey { command: api_cmd } => {
                    let val = commands::auth::handle_api_key(&client, api_cmd).await?;
                    let kind = match api_cmd {
                        ApiKeyCommand::List => OutputKind::ApiKeyList,
                        ApiKeyCommand::Create { .. } => OutputKind::ApiKey,
                        ApiKeyCommand::Revoke { .. } => OutputKind::BoolResult {
                            action: "API key revoked.",
                        },
                        ApiKeyCommand::Delete { .. } => OutputKind::BoolResult {
                            action: "API key deleted.",
                        },
                    };
                    Ok(CommandOutput::new(val, kind))
                }
            }
        }

        Command::Channel { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            let val = commands::channel::handle(&client, command).await?;
            let kind = match command {
                ChannelCommand::Get { .. } => OutputKind::ChannelDetail,
                ChannelCommand::List { .. } => OutputKind::ChannelList,
                ChannelCommand::Create { .. } => OutputKind::ChannelResult { action: "created" },
                ChannelCommand::Update { .. } => OutputKind::ChannelResult { action: "updated" },
                ChannelCommand::AddNotice { .. } | ChannelCommand::UpdateNotice { .. } => {
                    OutputKind::ChannelNotice
                }
                ChannelCommand::DeleteNotice { .. } => OutputKind::BoolResult {
                    action: "Channel notice deleted.",
                },
                ChannelCommand::Delete { .. } => OutputKind::ChannelResult { action: "deleted" },
                ChannelCommand::AddMember { .. } => {
                    OutputKind::ChannelMember { action: "added to" }
                }
                ChannelCommand::RemoveMember { .. } => OutputKind::ChannelMember {
                    action: "removed from",
                },
                ChannelCommand::AddGithubOidc { .. } => OutputKind::GithubPublisher,
                ChannelCommand::AddGitlabOidc { .. } => OutputKind::GitlabPublisher,
                ChannelCommand::AddGoogleOidc { .. } => OutputKind::GooglePublisher,
                ChannelCommand::DeleteOidc { .. } => OutputKind::OidcDeleted,
                ChannelCommand::Transfer { .. } => OutputKind::ChannelResult {
                    action: "transferred",
                },
            };
            Ok(CommandOutput::new(val, kind))
        }

        Command::Package { command } => {
            let client = PrefixClient::new(cli.endpoint.clone(), cli.token.clone());
            let val = commands::package::handle(&client, command).await?;
            let kind = match command {
                PackageCommand::Get { .. } => OutputKind::PackageDetail,
                PackageCommand::Search { .. } => OutputKind::PackageList,
                PackageCommand::List { .. } => OutputKind::PackageList,
                PackageCommand::Matchspec { .. } => OutputKind::PackageInfo,
                PackageCommand::Variant { .. } => OutputKind::VariantDetail,
                PackageCommand::Versions { .. } => OutputKind::PackageVersions,
                PackageCommand::Yank { .. } => OutputKind::BoolResult {
                    action: "Package variant yanked.",
                },
                PackageCommand::Unyank { .. } => OutputKind::BoolResult {
                    action: "Package variant unyanked.",
                },
                PackageCommand::BatchYank { .. } => OutputKind::BoolResult {
                    action: "Package variants yanked.",
                },
                PackageCommand::BatchUnyank { .. } => OutputKind::BoolResult {
                    action: "Package variants unyanked.",
                },
                PackageCommand::Copy { execution, .. }
                | PackageCommand::CopyFromChannel { execution, .. }
                    if execution.dry_run =>
                {
                    OutputKind::Raw
                }
                PackageCommand::Copy { .. }
                | PackageCommand::CopyFromChannel { .. }
                | PackageCommand::CopyStatus { .. }
                | PackageCommand::ActiveCopy { .. } => OutputKind::BackgroundJob,
                PackageCommand::BatchDelete { .. } => OutputKind::BoolResult {
                    action: "Package variants deleted.",
                },
            };
            Ok(CommandOutput::new(val, kind))
        }
    }
}
