use serde::Serialize;

#[allow(unused_imports)]
use crate::schema;
use super::common::DateTime;

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
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "CreateApiKeyVars")]
pub struct CreateApiKeyMutation {
    #[arguments(name: $name, description: $description, expiresAt: $expires_at)]
    pub create_api_key: ApiKeyResult,
}

#[derive(cynic::QueryVariables)]
pub struct ApiKeyNameVars {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "ApiKeyNameVars")]
pub struct RevokeApiKeyMutation {
    #[arguments(name: $name)]
    pub revoke_api_key: bool,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "ApiKeyNameVars")]
pub struct DeleteApiKeyMutation {
    #[arguments(name: $name)]
    pub delete_api_key: bool,
}
