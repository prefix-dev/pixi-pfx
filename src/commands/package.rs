use cynic::{MutationBuilder, QueryBuilder};
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::cli::{
    CopyWaitArgs, JobCommand, JobType, PackageCommand, PackageOrderField, SortDirection,
};
use crate::client::PrefixClient;
use crate::error::PfxError;
use crate::queries::common::*;
use crate::queries::package::*;

pub async fn handle_job(client: &PrefixClient, command: &JobCommand) -> Result<Value, PfxError> {
    match command {
        JobCommand::Get { id, wait } => {
            let job = get_background_job(client, id).await?;
            if wait.wait {
                Ok(serde_json::to_value(
                    wait_for_background_job(client, job, wait).await?,
                )?)
            } else {
                Ok(serde_json::to_value(job)?)
            }
        }
        JobCommand::Active { channel, job_type } => {
            let op = ActiveCopyJobQuery::build(ActiveCopyJobVars {
                channel_name: channel.clone(),
                job_type: job_type.map(|job_type| match job_type {
                    JobType::BatchDeletePackages => BackgroundJobType::BatchDeletePackages,
                    JobType::CopyPackagesFromUrl => BackgroundJobType::CopyPackagesFromUrl,
                }),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.active_background_job)?)
        }
    }
}

pub async fn handle(client: &PrefixClient, command: &PackageCommand) -> Result<Value, PfxError> {
    match command {
        PackageCommand::Get {
            channel,
            name,
            variants_page,
            variants_limit,
        } => {
            let op = PackageGetQuery::build(PackageGetVars {
                channel_name: channel.clone(),
                name: name.clone(),
                variants_limit: Some(*variants_limit),
                variants_page: Some(*variants_page),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.package)?)
        }

        PackageCommand::Search { query, limit, page } => {
            let op = PackageSearchQuery::build(PackageSearchVars {
                order_by: Some(PackageOrderBy {
                    by_field: None,
                    by_similarity: Some(PackageOrderBySimilarity {
                        field: PackageOrderBySimilarityField::Name,
                        direction: OrderDirection::Desc,
                        matches: query.clone(),
                    }),
                }),
                limit: Some(*limit),
                page: Some(*page),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.packages)?)
        }

        PackageCommand::List {
            name_contains,
            limit,
            page,
            order_by,
            direction,
        } => {
            let filters = name_contains.as_ref().map(|name| PackageFilter {
                or: None,
                and: None,
                name: Some(StringFilter {
                    eq: None,
                    ne: None,
                    is_in: None,
                    is_not_in: None,
                    starts_with: None,
                    ends_with: None,
                    contains: Some(name.clone()),
                }),
            });

            let order = order_by.map(|field| PackageOrderBy {
                by_field: Some(PackageOrderByField {
                    field: match field {
                        PackageOrderField::Name => PackageOrderByFieldField::Name,
                        PackageOrderField::LastCreatedDate => {
                            PackageOrderByFieldField::LastCreatedDate
                        }
                        PackageOrderField::TotalSize => PackageOrderByFieldField::TotalSize,
                    },
                    direction: match direction {
                        SortDirection::Asc => OrderDirection::Asc,
                        SortDirection::Desc => OrderDirection::Desc,
                    },
                }),
                by_similarity: None,
            });

            let op = PackageListQuery::build(PackageListVars {
                filters,
                order_by: order,
                limit: Some(*limit),
                page: Some(*page),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.packages)?)
        }

        PackageCommand::Matchspec { spec, channels } => {
            let op = PackageMatchspecQuery::build(PackageMatchspecVars {
                match_spec: spec.clone(),
                channels: channels.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.package_by_matchspec)?)
        }

        PackageCommand::Variant {
            channel,
            package,
            platform,
            filename,
        } => {
            let op = VariantGetQuery::build(VariantGetVars {
                channel_name: channel.clone(),
                package_name: package.clone(),
                platform_name: platform.clone(),
                file_name: filename.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.variant)?)
        }

        PackageCommand::Versions {
            channel,
            name,
            limit,
            page,
        } => {
            let op = PackageVersionsQuery::build(PackageVersionsVars {
                channel_name: channel.clone(),
                name: name.clone(),
                limit: Some(*limit),
                page: Some(*page),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.package)?)
        }

        PackageCommand::Yank {
            channel,
            subdir,
            filename,
            reason,
        } => {
            let op = YankMutation::build(YankVars {
                channel_name: channel.clone(),
                entries: vec![PackageVariantInput {
                    subdir: subdir.clone(),
                    filename: filename.clone(),
                }],
                reason: reason.clone(),
                also_hide: None,
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.batch_yank_package_variants)?)
        }

        PackageCommand::Unyank {
            channel,
            subdir,
            filename,
        } => {
            let op = UnyankMutation::build(UnyankVars {
                channel_name: channel.clone(),
                entries: vec![PackageVariantInput {
                    subdir: subdir.clone(),
                    filename: filename.clone(),
                }],
                also_unhide: None,
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.batch_unyank_package_variants)?)
        }

        PackageCommand::BatchYank {
            channel,
            entries,
            reason,
            also_hide,
        } => {
            let entries = parse_variant_entries(entries)?;
            let op = YankMutation::build(YankVars {
                channel_name: channel.clone(),
                entries,
                reason: reason.clone(),
                also_hide: Some(*also_hide),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.batch_yank_package_variants)?)
        }

        PackageCommand::BatchUnyank {
            channel,
            entries,
            also_unhide,
        } => {
            let entries = parse_variant_entries(entries)?;
            let op = UnyankMutation::build(UnyankVars {
                channel_name: channel.clone(),
                entries,
                also_unhide: Some(*also_unhide),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.batch_unyank_package_variants)?)
        }

        PackageCommand::Copy {
            channel,
            packages,
            package,
            execution,
        } => {
            let parsed = if let Some(packages) = packages {
                serde_json::from_str(packages)
                    .map_err(|e| PfxError::InvalidArgument(format!("Invalid packages JSON: {e}")))?
            } else {
                package
                    .iter()
                    .map(|value| parse_pinned_package(value))
                    .collect::<Result<Vec<_>, _>>()?
            };
            validate_copy_packages(&parsed)?;
            if execution.dry_run {
                return Ok(serde_json::to_value(parsed)?);
            }
            execute_copy(client, channel, parsed, &execution.wait).await
        }

        PackageCommand::CopyFromChannel {
            destination,
            source,
            packages,
            version,
            platform,
            execution,
        } => {
            let packages = resolve_channel_packages(
                client,
                source,
                packages,
                version.as_ref(),
                platform.as_ref(),
            )
            .await?;
            if execution.dry_run {
                return Ok(serde_json::to_value(packages)?);
            }
            execute_copy(client, destination, packages, &execution.wait).await
        }

        PackageCommand::CopyStatus { id, wait } => {
            let job = get_background_job(client, id).await?;
            if wait.wait {
                Ok(serde_json::to_value(
                    wait_for_background_job(client, job, wait).await?,
                )?)
            } else {
                Ok(serde_json::to_value(job)?)
            }
        }

        PackageCommand::ActiveCopy { channel } => {
            let op = ActiveCopyJobQuery::build(ActiveCopyJobVars {
                channel_name: channel.clone(),
                job_type: Some(BackgroundJobType::CopyPackagesFromUrl),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.active_background_job)?)
        }

        PackageCommand::BatchDelete { channel, entries } => {
            let parsed = parse_variant_entries(entries)?;
            let op = BatchDeleteMutation::build(BatchDeleteVars {
                channel_name: channel.clone(),
                entries: parsed,
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.batch_delete_package_variants)?)
        }
    }
}

fn parse_variant_entries(value: &str) -> Result<Vec<PackageVariantInput>, PfxError> {
    let entries: Vec<PackageVariantInput> = serde_json::from_str(value)
        .map_err(|e| PfxError::InvalidArgument(format!("Invalid entries JSON: {e}")))?;
    if entries.is_empty() {
        return Err(PfxError::InvalidArgument(
            "entries must contain at least one item".to_string(),
        ));
    }
    Ok(entries)
}

fn parse_pinned_package(value: &str) -> Result<CopyPackageUrlInput, PfxError> {
    let (url, sha256) = value.rsplit_once('=').ok_or_else(|| {
        PfxError::InvalidArgument(format!(
            "invalid pinned package '{value}'; expected URL=SHA256"
        ))
    })?;
    Ok(CopyPackageUrlInput {
        url: url.to_string(),
        sha256: sha256.to_string(),
    })
}

async fn execute_copy(
    client: &PrefixClient,
    destination: &str,
    packages: Vec<CopyPackageUrlInput>,
    wait: &CopyWaitArgs,
) -> Result<Value, PfxError> {
    let op = CopyPackagesMutation::build(CopyPackagesVars {
        channel_name: destination.to_string(),
        packages,
    });
    let data = client.execute(op).await?;
    let job = data.copy_packages_from_urls;
    if wait.wait {
        Ok(serde_json::to_value(
            wait_for_background_job(client, job, wait).await?,
        )?)
    } else {
        Ok(serde_json::to_value(job)?)
    }
}

async fn get_background_job(client: &PrefixClient, id: &str) -> Result<BackgroundJob, PfxError> {
    let op = BackgroundJobQuery::build(BackgroundJobVars { id: id.to_string() });
    let data = client.execute(op).await?;
    data.background_job
        .ok_or_else(|| PfxError::InvalidArgument(format!("background job '{id}' was not found")))
}

async fn wait_for_background_job(
    client: &PrefixClient,
    mut job: BackgroundJob,
    options: &CopyWaitArgs,
) -> Result<BackgroundJob, PfxError> {
    if options.poll_interval == 0 || options.timeout == 0 {
        return Err(PfxError::InvalidArgument(
            "poll interval and timeout must be greater than zero".to_string(),
        ));
    }
    let started = Instant::now();
    loop {
        match job.status {
            BackgroundJobStatus::Completed => return Ok(job),
            BackgroundJobStatus::CompletedWithErrors | BackgroundJobStatus::Failed => {
                let details = serde_json::to_value(&job).ok();
                return Err(PfxError::BackgroundJob {
                    message: job
                        .error_message
                        .clone()
                        .unwrap_or_else(|| format!("job '{}' finished with errors", job.id)),
                    details,
                });
            }
            BackgroundJobStatus::Pending | BackgroundJobStatus::InProgress => {}
        }

        if started.elapsed() >= Duration::from_secs(options.timeout) {
            return Err(PfxError::BackgroundJob {
                message: format!(
                    "timed out after {} seconds waiting for job '{}'",
                    options.timeout, job.id
                ),
                details: serde_json::to_value(&job).ok(),
            });
        }
        tokio::time::sleep(Duration::from_secs(options.poll_interval)).await;
        job = get_background_job(client, &job.id).await?;
    }
}

async fn resolve_channel_packages(
    client: &PrefixClient,
    source_channel: &str,
    package_names: &[String],
    version: Option<&String>,
    platform: Option<&String>,
) -> Result<Vec<CopyPackageUrlInput>, PfxError> {
    const PAGE_SIZE: i32 = 100;
    let mut inputs = Vec::new();

    for package_name in package_names {
        let mut page = 0;
        loop {
            let op = CopySourcePackageQuery::build(CopySourcePackageVars {
                channel_name: source_channel.to_string(),
                package_name: package_name.clone(),
                limit: Some(PAGE_SIZE),
                page: Some(page),
                version: version.cloned(),
                platform: platform.cloned(),
            });
            let data = client.execute(op).await?;
            let package = data.package.ok_or_else(|| {
                PfxError::InvalidArgument(format!(
                    "package '{package_name}' was not found in source channel '{source_channel}'"
                ))
            })?;
            let base_url = format!("{}/", package.channel.base_url.trim_end_matches('/'));
            let base_url = reqwest::Url::parse(&base_url).map_err(|error| {
                PfxError::InvalidArgument(format!(
                    "source channel returned an invalid base URL: {error}"
                ))
            })?;

            for variant in package.variants.page {
                let sha256 = variant.sha256.ok_or_else(|| {
                    PfxError::InvalidArgument(format!(
                        "source variant '{}/{}' has no SHA-256 digest",
                        variant.platform, variant.filename
                    ))
                })?;
                let url = base_url
                    .join(&format!("{}/{}", variant.platform, variant.filename))
                    .map_err(|error| {
                        PfxError::InvalidArgument(format!(
                            "could not construct source URL for '{}/{}': {error}",
                            variant.platform, variant.filename
                        ))
                    })?;
                inputs.push(CopyPackageUrlInput {
                    url: url.to_string(),
                    sha256,
                });
            }

            page += 1;
            if page >= package.variants.pages {
                break;
            }
        }
    }

    if inputs.is_empty() {
        return Err(PfxError::InvalidArgument(format!(
            "no package variants matched in source channel '{source_channel}'"
        )));
    }
    validate_copy_packages(&inputs)?;
    Ok(inputs)
}

fn validate_copy_packages(packages: &[CopyPackageUrlInput]) -> Result<(), PfxError> {
    if packages.is_empty() {
        return Err(PfxError::InvalidArgument(
            "packages must contain at least one item".to_string(),
        ));
    }

    for (index, package) in packages.iter().enumerate() {
        let url = reqwest::Url::parse(&package.url).map_err(|error| {
            PfxError::InvalidArgument(format!("packages[{index}].url is invalid: {error}"))
        })?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(PfxError::InvalidArgument(format!(
                "packages[{index}].url must use HTTP or HTTPS"
            )));
        }
        if package.sha256.len() != 64
            || !package.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err(PfxError::InvalidArgument(format!(
                "packages[{index}].sha256 must be exactly 64 hexadecimal characters"
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_copy_package_inputs() {
        let valid = vec![CopyPackageUrlInput {
            url: "https://prefix.dev/conda-forge/linux-64/example-1.0.conda".to_string(),
            sha256: "a".repeat(64),
        }];
        assert!(validate_copy_packages(&valid).is_ok());

        let invalid_hash = vec![CopyPackageUrlInput {
            url: "https://prefix.dev/example.conda".to_string(),
            sha256: "not-a-sha256".to_string(),
        }];
        assert!(validate_copy_packages(&invalid_hash).is_err());

        let invalid_scheme = vec![CopyPackageUrlInput {
            url: "file:///tmp/example.conda".to_string(),
            sha256: "a".repeat(64),
        }];
        assert!(validate_copy_packages(&invalid_scheme).is_err());
    }

    #[test]
    fn parses_repeated_pinned_package_syntax() {
        let pin = format!(
            "https://example.com/pkg.conda?download=1={}",
            "a".repeat(64)
        );
        let parsed = parse_pinned_package(&pin).unwrap();
        assert_eq!(parsed.url, "https://example.com/pkg.conda?download=1");
        assert_eq!(parsed.sha256, "a".repeat(64));
        assert!(parse_pinned_package("https://example.com/pkg.conda").is_err());
    }

    #[test]
    fn rejects_empty_variant_entry_lists() {
        assert!(parse_variant_entries("[]").is_err());
    }
}
