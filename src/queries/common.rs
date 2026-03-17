use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use crate::schema;

// ── Custom Scalars ──────────────────────────────────────────────────────────

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "DateTime")]
pub struct DateTime(pub String);

#[allow(dead_code)]
#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "NaiveDate")]
pub struct NaiveDate(pub String);

#[derive(cynic::Scalar, Debug, Clone)]
#[cynic(graphql_type = "JSON")]
pub struct Json(pub serde_json::Value);

// ── Enums ───────────────────────────────────────────────────────────────────
// Note: cynic::Enum already derives Serialize, so we don't derive it manually.

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelMemberRole {
    Owner,
    Contributor,
    Viewer,
    Member,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelOrderByFieldField {
    Name,
    Size,
    CreatedAt,
    PackageCount,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelOrderBySimilarityField {
    Name,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PackageOrderByFieldField {
    Name,
    LastCreatedDate,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PackageOrderBySimilarityField {
    Name,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UrlKind {
    Dev,
    Doc,
    Home,
    Source,
    Feedstock,
}

// ── Input Objects ───────────────────────────────────────────────────────────

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct StringFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub is_in: Option<Vec<String>>,
    pub is_not_in: Option<Vec<String>>,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    pub contains: Option<String>,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelFilter {
    pub or: Option<Vec<ChannelFilter>>,
    pub and: Option<Vec<ChannelFilter>>,
    pub name: Option<StringFilter>,
    pub owner: Option<StringFilter>,
    pub is_public: Option<bool>,
    pub has_owner: Option<bool>,
    pub has_mirror: Option<bool>,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelOrderBy {
    pub by_field: Option<ChannelOrderByField>,
    pub by_similarity: Option<ChannelOrderBySimilarity>,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelOrderByField {
    pub direction: OrderDirection,
    pub field: ChannelOrderByFieldField,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelOrderBySimilarity {
    pub direction: OrderDirection,
    pub field: ChannelOrderBySimilarityField,
    pub matches: String,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageFilter {
    pub or: Option<Vec<PackageFilter>>,
    pub and: Option<Vec<PackageFilter>>,
    pub name: Option<StringFilter>,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageOrderBy {
    pub by_field: Option<PackageOrderByField>,
    pub by_similarity: Option<PackageOrderBySimilarity>,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageOrderByField {
    pub direction: OrderDirection,
    pub field: PackageOrderByFieldField,
}

#[derive(cynic::InputObject, Debug, Clone)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageOrderBySimilarity {
    pub direction: OrderDirection,
    pub field: PackageOrderBySimilarityField,
    pub matches: String,
}

#[derive(cynic::InputObject, Debug, Clone, Deserialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageVariantInput {
    pub subdir: String,
    pub filename: String,
}

// ── Shared Fragments ────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct Url {
    pub url: String,
    pub kind: UrlKind,
    pub order: i32,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct ChannelNameOnly {
    pub name: String,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct ChannelMirror {
    pub url: String,
}
