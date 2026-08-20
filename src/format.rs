use std::io::Write;

use serde_json::{Value, json};
use tabwriter::TabWriter;

use crate::error::{ErrorResponse, PfxError};

pub enum OutputKind {
    Raw,
    User,
    ApiKeyList,
    ApiKey,
    BoolResult { action: &'static str },
    ChannelDetail,
    ChannelList,
    ChannelResult { action: &'static str },
    ChannelMember { action: &'static str },
    ChannelNotice,
    GithubPublisher,
    GitlabPublisher,
    GooglePublisher,
    OidcDeleted,
    PackageDetail,
    PackageList,
    PackageInfo,
    VariantDetail,
    PackageVersions,
    BackgroundJob,
}

pub struct CommandOutput {
    pub value: Value,
    pub kind: OutputKind,
}

impl CommandOutput {
    pub fn new(value: Value, kind: OutputKind) -> Self {
        Self { value, kind }
    }

    pub fn raw(value: Value) -> Self {
        Self {
            value,
            kind: OutputKind::Raw,
        }
    }
}

// ── JSON output ──────────────────────────────────────────────────────────────

pub fn format_json(output: &CommandOutput) -> String {
    let envelope = json!({ "ok": true, "data": output.value });
    serde_json::to_string(&envelope).unwrap()
}

pub fn format_json_error(err: &PfxError) -> String {
    let error_resp = ErrorResponse::from(err);
    let envelope = json!({ "ok": false, "error": error_resp });
    serde_json::to_string(&envelope).unwrap()
}

// ── Human-readable output ────────────────────────────────────────────────────

pub fn format_human_error(err: &PfxError) -> String {
    let resp = ErrorResponse::from(err);
    let mut out = format!("error [{}]: {}", resp.code, resp.message);
    if let Some(details) = resp.details {
        out.push_str(&format!(
            "\n{}",
            serde_json::to_string_pretty(&details).unwrap()
        ));
    }
    out
}

pub fn format_human(output: &CommandOutput) -> String {
    match &output.kind {
        OutputKind::Raw => {
            let mut s = serde_json::to_string_pretty(&output.value).unwrap();
            s.push('\n');
            s
        }
        OutputKind::User => format_user(&output.value),
        OutputKind::ApiKeyList => format_api_key_list(&output.value),
        OutputKind::ApiKey => format_api_key(&output.value),
        OutputKind::BoolResult { action } => format!("{action}\n"),
        OutputKind::ChannelDetail => format_channel_detail(&output.value),
        OutputKind::ChannelList => format_channel_list(&output.value),
        OutputKind::ChannelResult { action } => format_channel_result(&output.value, action),
        OutputKind::ChannelMember { action } => format_channel_member(&output.value, action),
        OutputKind::ChannelNotice => format_channel_notice(&output.value),
        OutputKind::GithubPublisher => format_github_publisher(&output.value),
        OutputKind::GitlabPublisher => format_gitlab_publisher(&output.value),
        OutputKind::GooglePublisher => format_google_publisher(&output.value),
        OutputKind::OidcDeleted => format_oidc_deleted(&output.value),
        OutputKind::PackageDetail => format_package_detail(&output.value),
        OutputKind::PackageList => format_package_list(&output.value),
        OutputKind::PackageInfo => format_package_info(&output.value),
        OutputKind::VariantDetail => format_variant_detail(&output.value),
        OutputKind::PackageVersions => format_package_versions(&output.value),
        OutputKind::BackgroundJob => format_background_job(&output.value),
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Extract a string field, returning "-" for null/missing.
fn sv(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string()
}

fn truncate(s: &str, max: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max {
        let truncated: String = chars[..max - 1].iter().collect();
        format!("{truncated}…")
    } else {
        s.to_string()
    }
}

fn yes_no(v: &Value, key: &str) -> &'static str {
    v.get(key)
        .and_then(|v| v.as_bool())
        .map_or("-", |b| if b { "yes" } else { "no" })
}

fn table(header: &[&str], rows: &[Vec<String>]) -> String {
    let mut tw = TabWriter::new(vec![]).padding(2);
    writeln!(&mut tw, "{}", header.join("\t")).unwrap();
    for row in rows {
        writeln!(&mut tw, "{}", row.join("\t")).unwrap();
    }
    tw.flush().unwrap();
    String::from_utf8(tw.into_inner().unwrap()).unwrap()
}

fn page_footer(v: &Value) -> String {
    let current = v.get("current").and_then(|v| v.as_i64()).unwrap_or(0);
    let pages = v.get("pages").and_then(|v| v.as_i64()).unwrap_or(0);
    let total = v.get("total_count").and_then(|v| v.as_i64()).unwrap_or(0);
    format!("Page {}/{} ({} total)\n", current + 1, pages, total)
}

fn kv(pairs: &[(&str, String)]) -> String {
    let mut tw = TabWriter::new(vec![]).padding(1);
    for (k, v) in pairs {
        writeln!(&mut tw, "{}:\t{}", k, v).unwrap();
    }
    tw.flush().unwrap();
    String::from_utf8(tw.into_inner().unwrap()).unwrap()
}

fn format_size(bytes: i64) -> String {
    const KB: i64 = 1024;
    const MB: i64 = 1024 * 1024;
    const GB: i64 = 1024 * 1024 * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

fn platforms_str(v: &Value) -> String {
    v.get("platforms")
        .and_then(|p| p.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "-".to_string())
}

fn nested_name(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|value| value.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("-")
        .to_string()
}

fn channel_name(v: &Value) -> String {
    v.get("channel")
        .and_then(|c| c.get("name"))
        .and_then(|n| n.as_str())
        .unwrap_or("-")
        .to_string()
}

fn latest_version(v: &Value) -> String {
    v.get("latest_version")
        .and_then(|lv| lv.get("version"))
        .and_then(|v| v.as_str())
        .unwrap_or("-")
        .to_string()
}

// ── Formatters ───────────────────────────────────────────────────────────────

fn format_user(v: &Value) -> String {
    if v.is_null() {
        "Not authenticated.\n".to_string()
    } else {
        format!("Logged in as: {}\n", sv(v, "login"))
    }
}

fn format_api_key_list(v: &Value) -> String {
    let keys = v.as_array().map(|a| a.as_slice()).unwrap_or(&[]);
    if keys.is_empty() {
        return "No API keys found.\n".to_string();
    }
    let rows: Vec<Vec<String>> = keys
        .iter()
        .map(|k| {
            vec![
                sv(k, "name"),
                truncate(&sv(k, "description"), 30),
                sv(k, "created_at"),
                sv(k, "expires_at"),
                sv(k, "last_used_at"),
                sv(k, "revoked_at"),
                sv(k, "access_mode"),
                nested_name(k, "channel"),
            ]
        })
        .collect();
    table(
        &[
            "NAME",
            "DESCRIPTION",
            "CREATED",
            "EXPIRES",
            "LAST USED",
            "REVOKED",
            "ACCESS",
            "CHANNEL",
        ],
        &rows,
    )
}

fn format_api_key(v: &Value) -> String {
    let mut out = kv(&[
        ("Name", sv(v, "name")),
        ("Description", sv(v, "description")),
        ("Created", sv(v, "created_at")),
        ("Expires", sv(v, "expires_at")),
        ("Access", sv(v, "access_mode")),
        ("Channel", nested_name(v, "channel")),
    ]);
    if let Some(key) = v.get("key").and_then(|v| v.as_str()) {
        out.push_str(&format!("\nAPI Key: {key}\n"));
        out.push_str("(save this now — it won't be shown again)\n");
    }
    out
}

fn format_channel_detail(v: &Value) -> String {
    if v.is_null() {
        return "Channel not found.\n".to_string();
    }
    let mut out = kv(&[
        ("Name", sv(v, "name")),
        ("Owner", nested_name(v, "owner")),
        ("Namespace", nested_name(v, "namespace")),
        ("Public", yes_no(v, "is_public").to_string()),
        ("Description", sv(v, "description")),
        ("Base URL", sv(v, "base_url")),
        ("Created", sv(v, "created_at")),
        ("Updated", sv(v, "updated_at")),
    ]);

    if let Some(notices) = v.get("notices").and_then(Value::as_array)
        && !notices.is_empty()
    {
        out.push('\n');
        let rows = notices
            .iter()
            .map(|notice| {
                vec![
                    sv(notice, "id"),
                    sv(notice, "level"),
                    sv(notice, "message"),
                    sv(notice, "expires_at"),
                ]
            })
            .collect::<Vec<_>>();
        out.push_str(&table(&["NOTICE", "LEVEL", "MESSAGE", "EXPIRES"], &rows));
    }

    if let Some(mirror) = v.get("mirror")
        && !mirror.is_null()
    {
        out.push_str(&format!("Mirror:   {}\n", sv(mirror, "url")));
    }

    if let Some(members) = v.get("channel_members").and_then(|v| v.as_array())
        && !members.is_empty()
    {
        out.push('\n');
        let rows: Vec<Vec<String>> = members
            .iter()
            .map(|m| {
                vec![
                    sv(m, "username"),
                    sv(m, "role"),
                    yes_no(m, "is_owner").to_string(),
                ]
            })
            .collect();
        out.push_str(&table(&["MEMBER", "ROLE", "OWNER"], &rows));
    }

    if let Some(publishers) = v.get("oidc_publishers").and_then(|v| v.as_array())
        && !publishers.is_empty()
    {
        out.push('\n');
        let rows: Vec<Vec<String>> = publishers.iter().map(format_publisher_row).collect();
        out.push_str(&table(&["TYPE", "ID", "DETAILS"], &rows));
    }

    out
}

fn format_publisher_row(p: &Value) -> Vec<String> {
    if let Some(gh) = p.get("GithubPublisher") {
        vec![
            "GitHub".to_string(),
            sv(gh, "id"),
            format!(
                "{}/{} ({})",
                sv(gh, "repository_owner"),
                sv(gh, "repository_name"),
                sv(gh, "workflow_filename"),
            ),
        ]
    } else if let Some(gl) = p.get("GitlabPublisher") {
        vec![
            "GitLab".to_string(),
            sv(gl, "id"),
            format!(
                "{}/{} ({})",
                sv(gl, "namespace"),
                sv(gl, "project"),
                sv(gl, "workflow_filepath"),
            ),
        ]
    } else if let Some(g) = p.get("GooglePublisher") {
        vec!["Google".to_string(), sv(g, "id"), sv(g, "email")]
    } else {
        vec!["Unknown".to_string(), "-".to_string(), "-".to_string()]
    }
}

fn format_channel_list(v: &Value) -> String {
    let page = v.get("page").and_then(|v| v.as_array());
    let items = page.map(|a| a.as_slice()).unwrap_or(&[]);
    if items.is_empty() {
        return "No channels found.\n".to_string();
    }
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|ch| {
            vec![
                sv(ch, "name"),
                nested_name(ch, "owner"),
                yes_no(ch, "is_public").to_string(),
                truncate(&sv(ch, "description"), 40),
            ]
        })
        .collect();
    let mut out = table(&["NAME", "OWNER", "PUBLIC", "DESCRIPTION"], &rows);
    out.push_str(&page_footer(v));
    out
}

fn format_channel_result(v: &Value, action: &str) -> String {
    format!("Channel '{}' {}.\n", sv(v, "name"), action)
}

fn format_channel_member(v: &Value, action: &str) -> String {
    format!(
        "Member '{}' {} channel '{}' (role: {}).\n",
        sv(v, "username"),
        action,
        nested_name(v, "channel"),
        sv(v, "role"),
    )
}

fn format_channel_notice(v: &Value) -> String {
    kv(&[
        ("ID", sv(v, "id")),
        ("Level", sv(v, "level")),
        ("Message", sv(v, "message")),
        ("Created", sv(v, "created_at")),
        ("Expires", sv(v, "expires_at")),
    ])
}

fn format_github_publisher(v: &Value) -> String {
    kv(&[
        ("Type", "GitHub".to_string()),
        ("ID", sv(v, "id")),
        ("Owner", sv(v, "repository_owner")),
        ("Repo", sv(v, "repository_name")),
        ("Workflow", sv(v, "workflow_filename")),
        ("Environment", sv(v, "environment")),
        ("Access", sv(v, "access_mode")),
        ("Created", sv(v, "created_at")),
    ])
}

fn format_gitlab_publisher(v: &Value) -> String {
    kv(&[
        ("Type", "GitLab".to_string()),
        ("ID", sv(v, "id")),
        ("Namespace", sv(v, "namespace")),
        ("Project", sv(v, "project")),
        ("Workflow", sv(v, "workflow_filepath")),
        ("Environment", sv(v, "environment")),
        ("Access", sv(v, "access_mode")),
        ("Created", sv(v, "created_at")),
    ])
}

fn format_google_publisher(v: &Value) -> String {
    kv(&[
        ("Type", "Google".to_string()),
        ("ID", sv(v, "id")),
        ("Email", sv(v, "email")),
        ("Subject", sv(v, "sub")),
        ("Access", sv(v, "access_mode")),
        ("Created", sv(v, "created_at")),
    ])
}

fn format_oidc_deleted(v: &Value) -> String {
    format!("OIDC publisher '{}' deleted.\n", sv(v, "id"))
}

fn format_background_job(v: &Value) -> String {
    if v.is_null() {
        return "Background job not found.\n".to_string();
    }

    let mut out = kv(&[
        ("ID", sv(v, "id")),
        ("Type", sv(v, "job_type")),
        ("Status", sv(v, "status")),
        (
            "Progress",
            format!(
                "{}/{} ({} failed)",
                v.get("processed_count")
                    .and_then(Value::as_i64)
                    .unwrap_or(0),
                v.get("total_count").and_then(Value::as_i64).unwrap_or(0),
                v.get("failed_count").and_then(Value::as_i64).unwrap_or(0),
            ),
        ),
        ("Created", sv(v, "created_at")),
        ("Completed", sv(v, "completed_at")),
        ("Error", sv(v, "error_message")),
    ]);
    if let Some(results) = v.get("results").filter(|results| !results.is_null()) {
        out.push_str("\nResults:\n");
        out.push_str(&serde_json::to_string_pretty(results).unwrap());
        out.push('\n');
    }
    out
}

fn format_package_detail(v: &Value) -> String {
    if v.is_null() {
        return "Package not found.\n".to_string();
    }
    let mut out = kv(&[
        ("Name", sv(v, "name")),
        ("Channel", channel_name(v)),
        ("Summary", sv(v, "summary")),
        ("Version", latest_version(v)),
        ("Platforms", platforms_str(v)),
        ("Verified", yes_no(v, "latest_version_verified").to_string()),
        ("Updated", sv(v, "last_created_date")),
    ]);

    if let Some(variants) = v.get("variants") {
        let page = variants.get("page").and_then(|v| v.as_array());
        if let Some(items) = page
            && !items.is_empty()
        {
            out.push('\n');
            let rows: Vec<Vec<String>> = items
                .iter()
                .map(|var| {
                    let size = var
                        .get("size")
                        .and_then(|s| s.as_i64())
                        .map(format_size)
                        .unwrap_or_else(|| "-".to_string());
                    vec![
                        sv(var, "filename"),
                        sv(var, "version"),
                        sv(var, "build_string"),
                        sv(var, "platform"),
                        size,
                    ]
                })
                .collect();
            out.push_str(&table(
                &["FILENAME", "VERSION", "BUILD", "PLATFORM", "SIZE"],
                &rows,
            ));
            out.push_str(&page_footer(variants));
        }
    }

    out
}

fn format_package_list(v: &Value) -> String {
    let page = v.get("page").and_then(|v| v.as_array());
    let items = page.map(|a| a.as_slice()).unwrap_or(&[]);
    if items.is_empty() {
        return "No packages found.\n".to_string();
    }
    let rows: Vec<Vec<String>> = items
        .iter()
        .map(|pkg| {
            vec![
                sv(pkg, "name"),
                channel_name(pkg),
                latest_version(pkg),
                truncate(&platforms_str(pkg), 30),
                truncate(&sv(pkg, "summary"), 40),
            ]
        })
        .collect();
    let mut out = table(
        &["NAME", "CHANNEL", "VERSION", "PLATFORMS", "SUMMARY"],
        &rows,
    );
    out.push_str(&page_footer(v));
    out
}

fn format_package_info(v: &Value) -> String {
    if v.is_null() {
        return "No matching package found.\n".to_string();
    }
    kv(&[
        ("Name", sv(v, "name")),
        ("Channel", channel_name(v)),
        ("Summary", sv(v, "summary")),
        ("Version", latest_version(v)),
        ("Platforms", platforms_str(v)),
        ("Verified", yes_no(v, "latest_version_verified").to_string()),
        ("Updated", sv(v, "last_created_date")),
    ])
}

fn format_variant_detail(v: &Value) -> String {
    if v.is_null() {
        return "Variant not found.\n".to_string();
    }
    let size = v
        .get("size")
        .and_then(|s| s.as_i64())
        .map(format_size)
        .unwrap_or_else(|| "-".to_string());
    let build_num = v
        .get("build_number")
        .and_then(|n| n.as_i64())
        .map(|n| n.to_string())
        .unwrap_or_else(|| "-".to_string());

    let mut pairs = vec![
        ("Filename", sv(v, "filename")),
        ("Version", sv(v, "version")),
        ("Build", sv(v, "build_string")),
        ("Build #", build_num),
        ("Platform", sv(v, "platform")),
        ("Size", size),
        ("License", sv(v, "license")),
        ("Summary", sv(v, "summary")),
        ("Created", sv(v, "created_at")),
        ("Updated", sv(v, "updated_at")),
    ];

    if let Some(reason) = v.get("yanked_reason").and_then(|r| r.as_str()) {
        pairs.push(("Yanked", reason.to_string()));
    }
    if let Some(md5) = v.get("md5").and_then(|h| h.as_str()) {
        pairs.push(("MD5", md5.to_string()));
    }
    if let Some(sha) = v.get("sha256").and_then(|h| h.as_str()) {
        pairs.push(("SHA256", sha.to_string()));
    }
    if let Some(downloads) = v.get("total_downloads").and_then(|d| d.as_i64()) {
        pairs.push(("Downloads", downloads.to_string()));
    }

    kv(&pairs)
}

fn format_package_versions(v: &Value) -> String {
    if v.is_null() {
        return "Package not found.\n".to_string();
    }
    let mut out = format!("{} ({})\n\n", sv(v, "name"), channel_name(v));

    if let Some(versions) = v.get("versions") {
        let page = versions.get("page").and_then(|v| v.as_array());
        if let Some(items) = page {
            if items.is_empty() {
                out.push_str("No versions found.\n");
            } else {
                let rows: Vec<Vec<String>> = items
                    .iter()
                    .map(|ver| {
                        let count = ver
                            .get("total_count")
                            .and_then(|c| c.as_i64())
                            .map(|c| c.to_string())
                            .unwrap_or_else(|| "-".to_string());
                        vec![sv(ver, "version"), truncate(&platforms_str(ver), 40), count]
                    })
                    .collect();
                out.push_str(&table(&["VERSION", "PLATFORMS", "VARIANTS"], &rows));
                out.push_str(&page_footer(versions));
            }
        }
    }

    out
}
