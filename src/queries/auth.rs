use serde::Serialize;

use super::common::{ChannelAccessMode, ChannelNameOnly, DateTime};
#[allow(unused_imports)]
use crate::schema;

// ── Fragments ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "User")]
pub struct UserInfo {
    pub login: String,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct ApiKeyResult {
    pub key: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub expires_at: Option<DateTime>,
    pub last_used_at: Option<DateTime>,
    pub revoked_at: Option<DateTime>,
    pub access_mode: ChannelAccessMode,
    pub channel: Option<ChannelNameOnly>,
}

// ── Queries ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query")]
pub struct ViewerQuery {
    pub viewer: Option<UserInfo>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query")]
pub struct ApiKeysQuery {
    pub api_keys: Vec<ApiKeyResult>,
}

// ── Mutations ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables)]
pub struct CreateApiKeyVars {
    pub name: String,
    pub description: Option<String>,
    pub expires_at: Option<DateTime>,
    pub access_mode: Option<ChannelAccessMode>,
    pub channel_name: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "CreateApiKeyVars"
)]
pub struct CreateApiKeyMutation {
    #[arguments(
        name: $name,
        description: $description,
        expiresAt: $expires_at,
        accessMode: $access_mode,
        channelName: $channel_name,
    )]
    pub create_api_key: ApiKeyResult,
}

#[derive(cynic::QueryVariables)]
pub struct ApiKeyNameVars {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "ApiKeyNameVars"
)]
pub struct RevokeApiKeyMutation {
    #[arguments(name: $name)]
    pub revoke_api_key: bool,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "ApiKeyNameVars"
)]
pub struct DeleteApiKeyMutation {
    #[arguments(name: $name)]
    pub delete_api_key: bool,
}

#[cfg(test)]
mod tests {
    use cynic::MutationBuilder;

    use super::*;

    #[test]
    fn scoped_api_key_variables_include_access_and_channel() {
        let operation = CreateApiKeyMutation::build(CreateApiKeyVars {
            name: "upload".to_string(),
            description: None,
            expires_at: None,
            access_mode: Some(ChannelAccessMode::ReadWrite),
            channel_name: Some("releases".to_string()),
        });
        let value = serde_json::to_value(operation).unwrap();
        assert_eq!(value["variables"]["accessMode"], "READ_WRITE");
        assert_eq!(value["variables"]["channelName"], "releases");
    }
}
