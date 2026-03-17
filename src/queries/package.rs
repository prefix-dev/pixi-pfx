use serde::Serialize;

#[allow(unused_imports)]
use crate::schema;
use super::common::*;

// ── Fragments ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageVersion {
    pub version: String,
    pub platforms: Vec<String>,
    pub urls: Vec<Url>,
    pub total_count: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "PackageVersionPageResult")]
pub struct PackageVersionPage {
    pub page: Vec<PackageVersion>,
    pub current: i32,
    pub pages: i32,
    pub total_count: i32,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct PackageVariant {
    pub filename: String,
    pub build_string: String,
    pub build_number: i32,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub license: Option<String>,
    pub license_family: Option<String>,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub version: String,
    pub platform: String,
    pub size: Option<i32>,
    pub md5: Option<String>,
    pub sha256: Option<String>,
    pub yanked_reason: Option<String>,
    pub raw_index: Json,
    pub raw_about: Option<Json>,
    pub raw_run_exports: Option<Json>,
    pub repo_data_patches: Option<Json>,
    pub urls: Vec<Url>,
    pub total_downloads: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "PackageVariantPageResult")]
pub struct PackageVariantPage {
    pub page: Vec<PackageVariant>,
    pub current: i32,
    pub pages: i32,
    pub total_count: i32,
}

/// Full package detail with variants (used by `package get`)
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Package", variables = "PackageGetVars")]
pub struct PackageDetail {
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub last_created_date: Option<DateTime>,
    pub platforms: Vec<String>,
    pub latest_version: PackageVersion,
    pub urls: Vec<Url>,
    pub latest_version_verified: bool,
    pub channel: ChannelNameOnly,
    #[arguments(limit: $variants_limit, page: $variants_page)]
    pub variants: PackageVariantPage,
}

/// Package info without nested collections (used by `matchspec`)
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Package")]
pub struct PackageInfo {
    pub name: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub last_created_date: Option<DateTime>,
    pub platforms: Vec<String>,
    pub latest_version: PackageVersion,
    pub urls: Vec<Url>,
    pub latest_version_verified: bool,
    pub channel: ChannelNameOnly,
}

/// Summary for search/list results
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Package")]
pub struct PackageSummary {
    pub name: String,
    pub summary: Option<String>,
    pub last_created_date: Option<DateTime>,
    pub platforms: Vec<String>,
    pub latest_version: PackageVersion,
    pub similarity_score: Option<f64>,
    pub channel: ChannelNameOnly,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "PackagePageResult")]
pub struct PackageSummaryPage {
    pub page: Vec<PackageSummary>,
    pub current: i32,
    pub pages: i32,
    pub total_count: i32,
}

/// Package with versions list
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Package", variables = "PackageVersionsVars")]
pub struct PackageWithVersions {
    pub name: String,
    pub channel: ChannelNameOnly,
    #[arguments(limit: $limit, page: $page)]
    pub versions: PackageVersionPage,
}

// ── Queries ─────────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables)]
pub struct PackageGetVars {
    pub channel_name: String,
    pub name: String,
    pub variants_limit: Option<i32>,
    pub variants_page: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "PackageGetVars")]
pub struct PackageGetQuery {
    #[arguments(channelName: $channel_name, name: $name)]
    pub package: Option<PackageDetail>,
}

#[derive(cynic::QueryVariables)]
pub struct PackageSearchVars {
    pub order_by: Option<PackageOrderBy>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "PackageSearchVars")]
pub struct PackageSearchQuery {
    #[arguments(orderBy: $order_by, limit: $limit, page: $page)]
    pub packages: PackageSummaryPage,
}

#[derive(cynic::QueryVariables)]
pub struct PackageListVars {
    pub filters: Option<PackageFilter>,
    pub order_by: Option<PackageOrderBy>,
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "PackageListVars")]
pub struct PackageListQuery {
    #[arguments(filters: $filters, orderBy: $order_by, limit: $limit, page: $page)]
    pub packages: PackageSummaryPage,
}

#[derive(cynic::QueryVariables)]
pub struct PackageMatchspecVars {
    pub match_spec: String,
    pub channels: Vec<String>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "PackageMatchspecVars")]
pub struct PackageMatchspecQuery {
    #[arguments(matchSpec: $match_spec, channels: $channels)]
    pub package_by_matchspec: Option<PackageInfo>,
}

#[derive(cynic::QueryVariables)]
pub struct VariantGetVars {
    pub channel_name: String,
    pub package_name: String,
    pub platform_name: String,
    pub file_name: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "VariantGetVars")]
pub struct VariantGetQuery {
    #[arguments(
        channelName: $channel_name,
        packageName: $package_name,
        platformName: $platform_name,
        fileName: $file_name,
    )]
    pub variant: Option<PackageVariant>,
}

#[derive(cynic::QueryVariables)]
pub struct PackageVersionsVars {
    pub channel_name: String,
    pub name: String,
    pub limit: Option<i32>,
    pub page: Option<i32>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Query", variables = "PackageVersionsVars")]
pub struct PackageVersionsQuery {
    #[arguments(channelName: $channel_name, name: $name)]
    pub package: Option<PackageWithVersions>,
}

// ── Mutations ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables)]
pub struct YankVars {
    pub channel_name: String,
    pub subdir: String,
    pub filename: String,
    pub reason: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "YankVars")]
pub struct YankMutation {
    #[arguments(
        channelName: $channel_name,
        subdir: $subdir,
        filename: $filename,
        reason: $reason,
    )]
    pub yank_package_variant: bool,
}

#[derive(cynic::QueryVariables)]
pub struct UnyankVars {
    pub channel_name: String,
    pub subdir: String,
    pub filename: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "UnyankVars")]
pub struct UnyankMutation {
    #[arguments(channelName: $channel_name, subdir: $subdir, filename: $filename)]
    pub unyank_package_variant: bool,
}

#[derive(cynic::QueryVariables)]
pub struct BatchDeleteVars {
    pub channel_name: String,
    pub entries: Vec<PackageVariantInput>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Mutation", variables = "BatchDeleteVars")]
pub struct BatchDeleteMutation {
    #[arguments(channelName: $channel_name, entries: $entries)]
    pub batch_delete_package_variants: bool,
}
