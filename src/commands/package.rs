use cynic::{MutationBuilder, QueryBuilder};
use serde_json::Value;

use crate::cli::{PackageCommand, PackageOrderField, SortDirection};
use crate::client::PrefixClient;
use crate::error::PfxError;
use crate::queries::common::*;
use crate::queries::package::*;

pub async fn handle(
    client: &PrefixClient,
    command: &PackageCommand,
) -> Result<Value, PfxError> {
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
                subdir: subdir.clone(),
                filename: filename.clone(),
                reason: reason.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.yank_package_variant)?)
        }

        PackageCommand::Unyank {
            channel,
            subdir,
            filename,
        } => {
            let op = UnyankMutation::build(UnyankVars {
                channel_name: channel.clone(),
                subdir: subdir.clone(),
                filename: filename.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.unyank_package_variant)?)
        }

        PackageCommand::BatchDelete { channel, entries } => {
            let parsed: Vec<PackageVariantInput> = serde_json::from_str(entries)
                .map_err(|e| PfxError::InvalidArgument(format!("Invalid entries JSON: {e}")))?;
            let op = BatchDeleteMutation::build(BatchDeleteVars {
                channel_name: channel.clone(),
                entries: parsed,
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.batch_delete_package_variants)?)
        }
    }
}
