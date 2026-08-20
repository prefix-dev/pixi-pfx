use serde::Serialize;

use super::common::*;
#[allow(unused_imports)]
use crate::schema;

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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "PackageVersionPageResult"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "PackageVariantPageResult"
)]
pub struct PackageVariantPage {
    pub page: Vec<PackageVariant>,
    pub current: i32,
    pub pages: i32,
    pub total_count: i32,
}

/// Full package detail with variants (used by `package get`)
#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Package",
    variables = "PackageGetVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Package",
    variables = "PackageVersionsVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "PackageGetVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "PackageSearchVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "PackageListVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "PackageMatchspecVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "VariantGetVars"
)]
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
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "PackageVersionsVars"
)]
pub struct PackageVersionsQuery {
    #[arguments(channelName: $channel_name, name: $name)]
    pub package: Option<PackageWithVersions>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema_path = "schema.graphql", graphql_type = "Channel")]
pub struct CopySourceChannel {
    pub base_url: String,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(schema_path = "schema.graphql", graphql_type = "PackageVariant")]
pub struct CopySourceVariant {
    pub filename: String,
    pub platform: String,
    pub sha256: Option<String>,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "PackageVariantPageResult"
)]
pub struct CopySourceVariantPage {
    pub page: Vec<CopySourceVariant>,
    pub pages: i32,
}

#[derive(cynic::QueryFragment, Debug, Clone)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Package",
    variables = "CopySourcePackageVars"
)]
pub struct CopySourcePackage {
    pub channel: CopySourceChannel,
    #[arguments(limit: $limit, page: $page, version: $version, platform: $platform)]
    pub variants: CopySourceVariantPage,
}

#[derive(cynic::QueryVariables)]
pub struct CopySourcePackageVars {
    pub channel_name: String,
    pub package_name: String,
    pub limit: Option<i32>,
    pub page: Option<i32>,
    pub version: Option<String>,
    pub platform: Option<String>,
}

#[derive(cynic::QueryFragment, Debug)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "CopySourcePackageVars"
)]
pub struct CopySourcePackageQuery {
    #[arguments(channelName: $channel_name, name: $package_name)]
    pub package: Option<CopySourcePackage>,
}

// ── Mutations ───────────────────────────────────────────────────────────────

#[derive(cynic::QueryVariables)]
pub struct YankVars {
    pub channel_name: String,
    pub entries: Vec<PackageVariantInput>,
    pub reason: String,
    pub also_hide: Option<bool>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "YankVars"
)]
pub struct YankMutation {
    #[arguments(
        channelName: $channel_name,
        entries: $entries,
        reason: $reason,
        alsoHide: $also_hide,
    )]
    pub batch_yank_package_variants: bool,
}

#[derive(cynic::QueryVariables)]
pub struct UnyankVars {
    pub channel_name: String,
    pub entries: Vec<PackageVariantInput>,
    pub also_unhide: Option<bool>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "UnyankVars"
)]
pub struct UnyankMutation {
    #[arguments(channelName: $channel_name, entries: $entries, alsoUnhide: $also_unhide)]
    pub batch_unyank_package_variants: bool,
}

#[derive(cynic::QueryVariables)]
pub struct BatchDeleteVars {
    pub channel_name: String,
    pub entries: Vec<PackageVariantInput>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "BatchDeleteVars"
)]
pub struct BatchDeleteMutation {
    #[arguments(channelName: $channel_name, entries: $entries)]
    pub batch_delete_package_variants: bool,
}

// ── Package copies and background jobs ──────────────────────────────────────

#[derive(cynic::InputObject, Debug, Clone, serde::Deserialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct CopyPackageUrlInput {
    pub url: String,
    pub sha256: String,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackgroundJobStatus {
    Pending,
    InProgress,
    Completed,
    CompletedWithErrors,
    Failed,
}

#[derive(cynic::Enum, Debug, Clone, Copy, PartialEq, Eq)]
#[cynic(schema_path = "schema.graphql", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BackgroundJobType {
    BatchDeletePackages,
    CopyPackagesFromUrl,
}

#[derive(cynic::QueryFragment, Debug, Clone, Serialize)]
#[cynic(schema_path = "schema.graphql")]
pub struct BackgroundJob {
    pub id: String,
    pub job_type: BackgroundJobType,
    pub status: BackgroundJobStatus,
    pub payload: Json,
    pub total_count: i32,
    pub processed_count: i32,
    pub failed_count: i32,
    pub error_message: Option<String>,
    pub results: Option<Json>,
    pub created_at: DateTime,
    pub completed_at: Option<DateTime>,
}

#[derive(cynic::QueryVariables)]
pub struct CopyPackagesVars {
    pub channel_name: String,
    pub packages: Vec<CopyPackageUrlInput>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Mutation",
    variables = "CopyPackagesVars"
)]
pub struct CopyPackagesMutation {
    #[arguments(channelName: $channel_name, packages: $packages)]
    pub copy_packages_from_urls: BackgroundJob,
}

#[derive(cynic::QueryVariables)]
pub struct BackgroundJobVars {
    pub id: String,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "BackgroundJobVars"
)]
pub struct BackgroundJobQuery {
    #[arguments(id: $id)]
    pub background_job: Option<BackgroundJob>,
}

#[derive(cynic::QueryVariables)]
pub struct ActiveCopyJobVars {
    pub channel_name: String,
    pub job_type: Option<BackgroundJobType>,
}

#[derive(cynic::QueryFragment, Debug, Serialize)]
#[cynic(
    schema_path = "schema.graphql",
    graphql_type = "Query",
    variables = "ActiveCopyJobVars"
)]
pub struct ActiveCopyJobQuery {
    #[arguments(channelName: $channel_name, jobType: $job_type)]
    pub active_background_job: Option<BackgroundJob>,
}

#[cfg(test)]
mod tests {
    use cynic::{MutationBuilder, QueryBuilder};

    use super::*;

    #[test]
    fn copy_mutation_serializes_expected_variables() {
        let operation = CopyPackagesMutation::build(CopyPackagesVars {
            channel_name: "destination".to_string(),
            packages: vec![CopyPackageUrlInput {
                url: "https://example.com/linux-64/pkg.conda".to_string(),
                sha256: "a".repeat(64),
            }],
        });
        let value = serde_json::to_value(operation).unwrap();
        assert_eq!(value["variables"]["channelName"], "destination");
        assert_eq!(value["variables"]["packages"][0]["sha256"], "a".repeat(64));
        assert!(
            value["query"]
                .as_str()
                .unwrap()
                .contains("copyPackagesFromUrls")
        );
    }

    #[test]
    fn source_query_includes_copy_filters_and_pagination() {
        let operation = CopySourcePackageQuery::build(CopySourcePackageVars {
            channel_name: "source".to_string(),
            package_name: "numpy".to_string(),
            limit: Some(100),
            page: Some(2),
            version: Some("2.3.0".to_string()),
            platform: Some("linux-64".to_string()),
        });
        let value = serde_json::to_value(operation).unwrap();
        assert_eq!(value["variables"]["channelName"], "source");
        assert_eq!(value["variables"]["packageName"], "numpy");
        assert_eq!(value["variables"]["page"], 2);
        assert_eq!(value["variables"]["version"], "2.3.0");
        assert_eq!(value["variables"]["platform"], "linux-64");
    }
}
