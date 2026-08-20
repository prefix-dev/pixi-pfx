use cynic::{MutationBuilder, QueryBuilder};
use serde_json::Value;

use crate::cli::{
    AccessMode, ChannelCommand, ChannelOrderField, MemberRole, NoticeLevel, SortDirection,
};
use crate::client::PrefixClient;
use crate::error::PfxError;
use crate::queries::channel::*;
use crate::queries::common::*;

pub async fn handle(client: &PrefixClient, command: &ChannelCommand) -> Result<Value, PfxError> {
    match command {
        ChannelCommand::Get { name } => {
            let op = ChannelGetQuery::build(ChannelGetVars { name: name.clone() });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.channel)?)
        }

        ChannelCommand::List {
            owner,
            public,
            limit,
            page,
            order_by,
            direction,
            search,
        } => {
            let filters = if owner.is_some() || *public {
                Some(ChannelFilter {
                    or: None,
                    and: None,
                    name: None,
                    owner: owner.as_ref().map(|o| StringFilter {
                        eq: Some(o.clone()),
                        ne: None,
                        is_in: None,
                        is_not_in: None,
                        starts_with: None,
                        ends_with: None,
                        contains: None,
                    }),
                    namespace: None,
                    is_public: if *public { Some(true) } else { None },
                    has_owner: None,
                    has_mirror: None,
                })
            } else {
                None
            };

            let order = if let Some(search_query) = search {
                Some(ChannelOrderBy {
                    by_field: None,
                    by_similarity: Some(ChannelOrderBySimilarity {
                        field: ChannelOrderBySimilarityField::Name,
                        direction: OrderDirection::Desc,
                        matches: search_query.clone(),
                    }),
                })
            } else {
                order_by.map(|field| ChannelOrderBy {
                    by_field: Some(ChannelOrderByField {
                        field: match field {
                            ChannelOrderField::Name => ChannelOrderByFieldField::Name,
                            ChannelOrderField::BillingOwner => {
                                ChannelOrderByFieldField::BillingOwner
                            }
                            ChannelOrderField::Namespace => ChannelOrderByFieldField::Namespace,
                            ChannelOrderField::Size => ChannelOrderByFieldField::Size,
                            ChannelOrderField::CreatedAt => ChannelOrderByFieldField::CreatedAt,
                            ChannelOrderField::PackageCount => {
                                ChannelOrderByFieldField::PackageCount
                            }
                        },
                        direction: match direction {
                            SortDirection::Asc => OrderDirection::Asc,
                            SortDirection::Desc => OrderDirection::Desc,
                        },
                    }),
                    by_similarity: None,
                })
            };

            let op = ChannelListQuery::build(ChannelListVars {
                filters,
                order_by: order,
                limit: Some(*limit),
                page: Some(*page),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.channels)?)
        }

        ChannelCommand::Create {
            name,
            description,
            public,
            logo,
            relation_base,
            relation_overrides,
            allow_v3_uploads,
        } => {
            let op = CreateChannelMutation::build(CreateChannelVars {
                name: name.clone(),
                description: description.clone(),
                is_public: Some(*public),
                logo: logo.clone(),
                channel_relation_base: relation_base.clone(),
                channel_relation_overrides: relation_overrides.clone(),
                allow_v3_uploads: *allow_v3_uploads,
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.create_channel.channel)?)
        }

        ChannelCommand::Update {
            name,
            description,
            public,
            logo,
            relation_base,
            relation_overrides,
            allow_v3_uploads,
        } => {
            let op = UpdateChannelMutation::build(UpdateChannelVars {
                name: name.clone(),
                description: description.clone(),
                is_public: *public,
                logo: logo.clone(),
                channel_relation_base: relation_base.clone(),
                channel_relation_overrides: relation_overrides.clone(),
                allow_v3_uploads: *allow_v3_uploads,
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.update_channel.channel)?)
        }

        ChannelCommand::AddNotice {
            channel,
            id,
            message,
            level,
            expires_at,
        } => {
            let op = CreateChannelNoticeMutation::build(UpsertChannelNoticeVars {
                channel_name: channel.clone(),
                id: id.clone(),
                message: message.clone(),
                level: notice_level(*level),
                expires_at: expires_at.clone().map(DateTime),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.create_channel_notice)?)
        }

        ChannelCommand::UpdateNotice {
            channel,
            id,
            message,
            level,
            expires_at,
        } => {
            let op = UpdateChannelNoticeMutation::build(UpsertChannelNoticeVars {
                channel_name: channel.clone(),
                id: id.clone(),
                message: message.clone(),
                level: notice_level(*level),
                expires_at: expires_at.clone().map(DateTime),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.update_channel_notice)?)
        }

        ChannelCommand::DeleteNotice { channel, id } => {
            let op = DeleteChannelNoticeMutation::build(DeleteChannelNoticeVars {
                channel_name: channel.clone(),
                id: id.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.delete_channel_notice)?)
        }

        ChannelCommand::Delete { name } => {
            let op = DeleteChannelMutation::build(DeleteChannelVars { name: name.clone() });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.delete_channel.channel)?)
        }

        ChannelCommand::AddMember {
            channel,
            username,
            role,
        } => {
            let op = AddChannelMemberMutation::build(AddChannelMemberVars {
                user_name: username.clone(),
                channel_name: channel.clone(),
                role: match role {
                    MemberRole::Owner => ChannelMemberRole::Owner,
                    MemberRole::Contributor => ChannelMemberRole::Contributor,
                    MemberRole::Viewer => ChannelMemberRole::Viewer,
                },
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.add_channel_member)?)
        }

        ChannelCommand::RemoveMember { channel, username } => {
            let op = DeleteChannelMemberMutation::build(DeleteChannelMemberVars {
                user_name: username.clone(),
                channel_name: channel.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.delete_channel_member)?)
        }

        ChannelCommand::AddGithubOidc {
            channel,
            owner,
            repo,
            workflow,
            environment,
            access_mode,
        } => {
            let op = AddGithubOidcMutation::build(AddGithubOidcVars {
                channel_name: channel.clone(),
                repository_owner: owner.clone(),
                repository_name: repo.clone(),
                workflow_filename: workflow.clone(),
                environment: environment.clone(),
                access_mode: access_mode.map(channel_access_mode),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.add_github_oidc_publisher)?)
        }

        ChannelCommand::AddGitlabOidc {
            channel,
            namespace,
            project,
            workflow,
            environment,
            access_mode,
        } => {
            let op = AddGitlabOidcMutation::build(AddGitlabOidcVars {
                channel_name: channel.clone(),
                namespace: namespace.clone(),
                project: project.clone(),
                workflow_filepath: workflow.clone(),
                environment: environment.clone(),
                access_mode: access_mode.map(channel_access_mode),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.add_gitlab_oidc_publisher)?)
        }

        ChannelCommand::AddGoogleOidc {
            channel,
            email,
            sub,
            access_mode,
        } => {
            let op = AddGoogleOidcMutation::build(AddGoogleOidcVars {
                channel_name: channel.clone(),
                email: email.clone(),
                sub: sub.clone(),
                access_mode: access_mode.map(channel_access_mode),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.add_google_oidc_publisher)?)
        }

        ChannelCommand::DeleteOidc { channel, id } => {
            let op = DeleteOidcMutation::build(DeleteOidcVars {
                channel_name: channel.clone(),
                id: id.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(data.delete_oidc_publisher)?)
        }

        ChannelCommand::Transfer { channel, new_owner } => {
            let op = TransferChannelMutation::build(TransferChannelVars {
                channel_name: channel.clone(),
                new_owner_username: new_owner.clone(),
            });
            let data = client.execute(op).await?;
            Ok(serde_json::to_value(
                data.transfer_channel_ownership.channel,
            )?)
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

fn notice_level(level: NoticeLevel) -> ChannelNoticeLevel {
    match level {
        NoticeLevel::Info => ChannelNoticeLevel::Info,
        NoticeLevel::Warning => ChannelNoticeLevel::Warning,
        NoticeLevel::Critical => ChannelNoticeLevel::Critical,
    }
}
