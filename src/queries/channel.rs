use serde::Serialize;

use super::common::*;
#[allow(unused_imports)]
use crate::schema;

// ── Channel Fragments ───────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct ChannelDetail {
    pub name: String,
    pub channel_path: String,
    pub namespace: OwnerName,
    pub is_primary: bool,
    pub is_public: bool,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub base_url: String,
    pub owner: Option<OwnerName>,
    pub mirror: Option<ChannelMirror>,
    pub notices: Vec<ChannelNotice>,
    pub channel_relation_base: Option<ChannelNameOnly>,
    pub channel_relation_overrides: Option<ChannelNameOnly>,
    pub allow_v3_uploads: bool,
    pub channel_members: Vec<ChannelMemberInfo>,
    pub oidc_publishers: Vec<PublisherFragment>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct ChannelSummary {
    pub name: String,
    pub channel_path: String,
    pub namespace: OwnerName,
    pub is_public: bool,
    pub description: Option<String>,
    pub owner: Option<OwnerName>,
    pub created_at: DateTime,
    pub base_url: String,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "ChannelPageResult")]
pub struct ChannelPage {
    pub page: Vec<ChannelSummary>,
    pub current: i32,
    pub pages: i32,
    pub total_count: i32,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct ChannelMutationResult {
    pub name: String,
    pub channel_path: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_public: bool,
    pub allow_v3_uploads: bool,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelResult {
    pub channel: ChannelMutationResult,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelMember {
    pub username: String,
    pub channel: ChannelNameOnly,
    pub role: ChannelMemberRole,
    pub is_owner: bool,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelNoticeLevel {
    Info,
    Warning,
    Critical,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelNotice {
    pub id: String,
    pub message: String,
    pub level: ChannelNoticeLevel,
    pub created_at: Option<DateTime>,
    pub expires_at: Option<DateTime>,
}

// ── Publisher Fragments (interface) ─────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "ChannelMember")]
pub struct ChannelMemberInfo {
    pub username: String,
    pub role: ChannelMemberRole,
    pub is_owner: bool,
}

#[derive(cynic::InlineFragments, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Publisher")]
pub enum PublisherFragment {
    GithubPublisher(GithubPublisherDetail),
    GitlabPublisher(GitlabPublisherDetail),
    GooglePublisher(GooglePublisherDetail),
    #[cynic(fallback)]
    Unknown,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "GithubPublisher")]
pub struct GithubPublisherDetail {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub repository_name: String,
    pub repository_owner: String,
    pub workflow_filename: String,
    pub environment: String,
    pub access_mode: ChannelAccessMode,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "GitlabPublisher")]
pub struct GitlabPublisherDetail {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub namespace: String,
    pub project: String,
    pub workflow_filepath: String,
    pub environment: String,
    pub access_mode: ChannelAccessMode,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "GooglePublisher")]
pub struct GooglePublisherDetail {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub email: String,
    pub sub: String,
    pub access_mode: ChannelAccessMode,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct OidcPublisher {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

// ── Queries ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables)]
pub struct ChannelGetVars {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "ChannelGetVars"
)]
pub struct ChannelGetQuery {
    #[arguments(name: $name)]
    pub channel: Option<ChannelDetail>,
}

#[derive(cynic::QueryVariables)]
pub struct ChannelListVars {
    pub filters: Option<ChannelFilter>,
    pub order_by: Option<ChannelOrderBy>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "ChannelListVars"
)]
pub struct ChannelListQuery {
    #[arguments(filters: $filters, orderBy: $order_by, limit: $limit, page: $page)]
    pub channels: ChannelPage,
}

// ── Mutations ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables)]
pub struct CreateChannelVars {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub logo: Option<String>,
    pub channel_relation_base: Option<String>,
    pub channel_relation_overrides: Option<String>,
    pub allow_v3_uploads: Option<bool>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "CreateChannelVars"
)]
pub struct CreateChannelMutation {
    #[arguments(
        name: $name,
        description: $description,
        isPublic: $is_public,
        logo: $logo,
        channelRelationBase: $channel_relation_base,
        channelRelationOverrides: $channel_relation_overrides,
        allowV3Uploads: $allow_v3_uploads,
    )]
    pub create_channel: ChannelResult,
}

#[derive(cynic::QueryVariables)]
pub struct UpdateChannelVars {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub logo: Option<String>,
    pub channel_relation_base: Option<String>,
    pub channel_relation_overrides: Option<String>,
    pub allow_v3_uploads: Option<bool>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "UpdateChannelVars"
)]
pub struct UpdateChannelMutation {
    #[arguments(
        name: $name,
        description: $description,
        isPublic: $is_public,
        logo: $logo,
        channelRelationBase: $channel_relation_base,
        channelRelationOverrides: $channel_relation_overrides,
        allowV3Uploads: $allow_v3_uploads,
    )]
    pub update_channel: ChannelResult,
}

#[derive(cynic::QueryVariables)]
pub struct DeleteChannelVars {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "DeleteChannelVars"
)]
pub struct DeleteChannelMutation {
    #[arguments(name: $name)]
    pub delete_channel: ChannelResult,
}

#[derive(cynic::QueryVariables)]
pub struct AddChannelMemberVars {
    pub user_name: String,
    pub channel_name: String,
    pub role: ChannelMemberRole,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "AddChannelMemberVars"
)]
pub struct AddChannelMemberMutation {
    #[arguments(userName: $user_name, channelName: $channel_name, role: $role)]
    pub add_channel_member: ChannelMember,
}

#[derive(cynic::QueryVariables)]
pub struct DeleteChannelMemberVars {
    pub user_name: String,
    pub channel_name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "DeleteChannelMemberVars"
)]
pub struct DeleteChannelMemberMutation {
    #[arguments(userName: $user_name, channelName: $channel_name)]
    pub delete_channel_member: ChannelMember,
}

#[derive(cynic::QueryVariables)]
pub struct AddGithubOidcVars {
    pub channel_name: String,
    pub repository_owner: String,
    pub repository_name: String,
    pub workflow_filename: String,
    pub environment: Option<String>,
    pub access_mode: Option<ChannelAccessMode>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "AddGithubOidcVars"
)]
pub struct AddGithubOidcMutation {
    #[arguments(
        channelName: $channel_name,
        repositoryOwner: $repository_owner,
        repositoryName: $repository_name,
        workflowFilename: $workflow_filename,
        environment: $environment,
        accessMode: $access_mode,
    )]
    pub add_github_oidc_publisher: GithubPublisherDetail,
}

#[derive(cynic::QueryVariables)]
pub struct AddGitlabOidcVars {
    pub channel_name: String,
    pub namespace: String,
    pub project: String,
    pub workflow_filepath: String,
    pub environment: Option<String>,
    pub access_mode: Option<ChannelAccessMode>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "AddGitlabOidcVars"
)]
pub struct AddGitlabOidcMutation {
    #[arguments(
        channelName: $channel_name,
        namespace: $namespace,
        project: $project,
        workflowFilepath: $workflow_filepath,
        environment: $environment,
        accessMode: $access_mode,
    )]
    pub add_gitlab_oidc_publisher: GitlabPublisherDetail,
}

#[derive(cynic::QueryVariables)]
pub struct AddGoogleOidcVars {
    pub channel_name: String,
    pub email: String,
    pub sub: Option<String>,
    pub access_mode: Option<ChannelAccessMode>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "AddGoogleOidcVars"
)]
pub struct AddGoogleOidcMutation {
    #[arguments(channelName: $channel_name, email: $email, sub: $sub, accessMode: $access_mode)]
    pub add_google_oidc_publisher: GooglePublisherDetail,
}

#[derive(cynic::QueryVariables)]
pub struct DeleteOidcVars {
    pub channel_name: String,
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "DeleteOidcVars"
)]
pub struct DeleteOidcMutation {
    #[arguments(channelName: $channel_name, id: $id)]
    pub delete_oidc_publisher: OidcPublisher,
}

#[derive(cynic::QueryVariables)]
pub struct TransferChannelVars {
    pub channel_name: String,
    pub new_owner_username: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "TransferChannelVars"
)]
pub struct TransferChannelMutation {
    #[arguments(channelName: $channel_name, newOwnerUsername: $new_owner_username)]
    pub transfer_channel_ownership: ChannelResult,
}

#[derive(cynic::QueryVariables)]
pub struct UpsertChannelNoticeVars {
    pub channel_name: String,
    pub id: String,
    pub message: String,
    pub level: ChannelNoticeLevel,
    pub expires_at: Option<DateTime>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "UpsertChannelNoticeVars"
)]
pub struct CreateChannelNoticeMutation {
    #[arguments(channelName: $channel_name, id: $id, message: $message, level: $level, expiresAt: $expires_at)]
    pub create_channel_notice: ChannelNotice,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "UpsertChannelNoticeVars"
)]
pub struct UpdateChannelNoticeMutation {
    #[arguments(channelName: $channel_name, id: $id, message: $message, level: $level, expiresAt: $expires_at)]
    pub update_channel_notice: ChannelNotice,
}

#[derive(cynic::QueryVariables)]
pub struct DeleteChannelNoticeVars {
    pub channel_name: String,
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "DeleteChannelNoticeVars"
)]
pub struct DeleteChannelNoticeMutation {
    #[arguments(channelName: $channel_name, id: $id)]
    pub delete_channel_notice: bool,
}
