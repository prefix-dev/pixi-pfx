use serde::Serialize;

#[allow(unused_imports)]
use crate::schema;
use super::common::*;

// ── Channel Fragments ───────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct ChannelDetail {
    pub name: String,
    pub is_public: bool,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub base_url: String,
    pub owner: Option<String>,
    pub required_channels: Vec<String>,
    pub mirror: Option<ChannelMirror>,
    pub channel_members: Vec<ChannelMemberInfo>,
    pub oidc_publishers: Vec<PublisherFragment>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct ChannelSummary {
    pub name: String,
    pub is_public: bool,
    pub description: Option<String>,
    pub owner: Option<String>,
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
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelResult {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub is_public: bool,
    pub required_channels: Vec<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelMember {
    pub username: String,
    pub channel_name: String,
    pub role: ChannelMemberRole,
    pub is_owner: bool,
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
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "GooglePublisher")]
pub struct GooglePublisherDetail {
    pub id: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub email: String,
    pub sub: String,
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
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "ChannelGetVars")]
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
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "ChannelListVars")]
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
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "CreateChannelVars")]
pub struct CreateChannelMutation {
    #[arguments(name: $name, description: $description, isPublic: $is_public, logo: $logo)]
    pub create_channel: ChannelResult,
}

#[derive(cynic::QueryVariables)]
pub struct UpdateChannelVars {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
    pub logo: Option<String>,
    pub required_channels: Option<Vec<String>>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "UpdateChannelVars")]
pub struct UpdateChannelMutation {
    #[arguments(
        name: $name,
        description: $description,
        isPublic: $is_public,
        logo: $logo,
        requiredChannels: $required_channels,
    )]
    pub update_channel: ChannelResult,
}

#[derive(cynic::QueryVariables)]
pub struct DeleteChannelVars {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "DeleteChannelVars")]
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
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "AddChannelMemberVars")]
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
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "DeleteChannelMemberVars")]
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
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "AddGithubOidcVars")]
pub struct AddGithubOidcMutation {
    #[arguments(
        channelName: $channel_name,
        repositoryOwner: $repository_owner,
        repositoryName: $repository_name,
        workflowFilename: $workflow_filename,
        environment: $environment,
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
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "AddGitlabOidcVars")]
pub struct AddGitlabOidcMutation {
    #[arguments(
        channelName: $channel_name,
        namespace: $namespace,
        project: $project,
        workflowFilepath: $workflow_filepath,
        environment: $environment,
    )]
    pub add_gitlab_oidc_publisher: GitlabPublisherDetail,
}

#[derive(cynic::QueryVariables)]
pub struct AddGoogleOidcVars {
    pub channel_name: String,
    pub email: String,
    pub sub: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "AddGoogleOidcVars")]
pub struct AddGoogleOidcMutation {
    #[arguments(channelName: $channel_name, email: $email, sub: $sub)]
    pub add_google_oidc_publisher: GooglePublisherDetail,
}

#[derive(cynic::QueryVariables)]
pub struct DeleteOidcVars {
    pub channel_name: String,
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "DeleteOidcVars")]
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
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "TransferChannelVars")]
pub struct TransferChannelMutation {
    #[arguments(channelName: $channel_name, newOwnerUsername: $new_owner_username)]
    pub transfer_channel_ownership: ChannelResult,
}
