use cynic::{MutationBuilder, QueryBuilder};
use serde_json::Value;

use crate::cli::{AccessMode, ApiKeyCommand};
use crate::client::PrefixClient;
use crate::error::PfxError;
use crate::queries::auth::*;
use crate::queries::common::{ChannelAccessMode, DateTime};

pub async fn handle_whoami(client: &PrefixClient) -> Result<Value, PfxError> {
    let op = ViewerQuery::build(());
    let data = client.execute(op).await?;
    Ok(serde_json::to_value(data.viewer)?)
}

pub async fn handle_api_key(
    client: &PrefixClient,
    command: &ApiKeyCommand,
) -> Result<Value, PfxError> {
    match command {
        ApiKeyCommand::List => {
            let op = ApiKeysQuery::build(());
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.api_keys)?)
        }
        ApiKeyCommand::Create {
            name,
            description,
            expires_at,
            access_mode,
            channel,
        } => {
            match (access_mode, channel) {
                (Some(AccessMode::All), Some(_)) => {
                    return Err(PfxError::InvalidArgument(
                        "--channel cannot be combined with --access-mode all".to_string(),
                    ));
                }
                (Some(mode), None) if !matches!(mode, AccessMode::All) => {
                    return Err(PfxError::InvalidArgument(
                        "scoped access modes require --channel".to_string(),
                    ));
                }
                _ => {}
            }
            let op = CreateApiKeyMutation::build(CreateApiKeyVars {
                name: name.clone(),
                description: description.clone(),
                expires_at: expires_at.clone().map(DateTime),
                access_mode: access_mode.map(channel_access_mode),
                channel_name: channel.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.create_api_key)?)
        }
        ApiKeyCommand::Revoke { name } => {
            let op = RevokeApiKeyMutation::build(ApiKeyNameVars { name: name.clone() });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.revoke_api_key)?)
        }
        ApiKeyCommand::Delete { name } => {
            let op = DeleteApiKeyMutation::build(ApiKeyNameVars { name: name.clone() });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.delete_api_key)?)
        }
    }
}

fn channel_access_mode(mode: AccessMode) -> ChannelAccessMode {
    match mode {
        AccessMode::All => ChannelAccessMode::All,
        AccessMode::Read => ChannelAccessMode::Read,
        AccessMode::ReadWrite => ChannelAccessMode::ReadWrite,
        AccessMode::ReadWriteDelete => ChannelAccessMode::ReadWriteDelete,
    }
}
