use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "pixi-pfx", about = "prefix.dev GraphQL API client for pixi")]
pub struct Cli {
    /// API authentication token (or set PREFIX_DEV_API_TOKEN env var)
    #[arg(long, global = true, env = "PREFIX_DEV_API_TOKEN")]
    pub token: Option<String>,

    /// GraphQL endpoint URL
    #[arg(
        long,
        global = true,
        default_value = "https://prefix.dev/api/graphql"
    )]
    pub endpoint: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Manage channels
    Channel {
        #[command(subcommand)]
        command: ChannelCommand,
    },
    /// Manage packages
    Package {
        #[command(subcommand)]
        command: PackageCommand,
    },
    /// Authentication and API key management
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    /// Output JSON schema of all commands (for agent discovery)
    Describe {
        /// Optional path to a specific command (e.g. "channel get")
        #[arg(trailing_var_arg = true)]
        command_path: Vec<String>,
    },
}

// ── Channel Commands ────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum ChannelCommand {
    /// Get channel details by name
    Get {
        /// Channel name
        name: String,
    },
    /// List channels with filtering and pagination
    List {
        /// Filter by owner
        #[arg(long)]
        owner: Option<String>,
        /// Filter to only public channels
        #[arg(long)]
        public: bool,
        /// Number of results per page
        #[arg(long, default_value = "25")]
        limit: i32,
        /// Page number (0-indexed)
        #[arg(long, default_value = "0")]
        page: i32,
        /// Order by field
        #[arg(long, value_enum)]
        order_by: Option<ChannelOrderField>,
        /// Sort direction
        #[arg(long, value_enum, default_value = "asc")]
        direction: SortDirection,
        /// Search by name (similarity-based ordering)
        #[arg(long)]
        search: Option<String>,
    },
    /// Create a new channel
    Create {
        /// Channel name
        name: String,
        /// Channel description
        #[arg(long)]
        description: Option<String>,
        /// Make channel public
        #[arg(long)]
        public: bool,
        /// Channel logo URL
        #[arg(long)]
        logo: Option<String>,
    },
    /// Update an existing channel
    Update {
        /// Channel name
        name: String,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// Set public visibility
        #[arg(long)]
        public: Option<bool>,
        /// Logo URL
        #[arg(long)]
        logo: Option<String>,
        /// Required channels (comma-separated)
        #[arg(long, value_delimiter = ',')]
        required_channels: Option<Vec<String>>,
    },
    /// Delete a channel
    Delete {
        /// Channel name
        name: String,
    },
    /// Add a member to a channel
    AddMember {
        /// Channel name
        channel: String,
        /// Username to add
        username: String,
        /// Member role
        #[arg(value_enum)]
        role: MemberRole,
    },
    /// Remove a member from a channel
    RemoveMember {
        /// Channel name
        channel: String,
        /// Username to remove
        username: String,
    },
    /// Add a GitHub OIDC publisher to a channel
    AddGithubOidc {
        /// Channel name
        channel: String,
        /// Repository owner
        #[arg(long)]
        owner: String,
        /// Repository name
        #[arg(long)]
        repo: String,
        /// Workflow filename
        #[arg(long)]
        workflow: String,
        /// GitHub environment
        #[arg(long)]
        environment: Option<String>,
    },
    /// Add a GitLab OIDC publisher to a channel
    AddGitlabOidc {
        /// Channel name
        channel: String,
        /// GitLab namespace
        #[arg(long)]
        namespace: String,
        /// GitLab project name
        #[arg(long)]
        project: String,
        /// Workflow filepath
        #[arg(long)]
        workflow: String,
        /// GitLab environment
        #[arg(long)]
        environment: Option<String>,
    },
    /// Add a Google OIDC publisher to a channel
    AddGoogleOidc {
        /// Channel name
        channel: String,
        /// Google service account email
        #[arg(long)]
        email: String,
        /// Optional subject constraint
        #[arg(long)]
        sub: Option<String>,
    },
    /// Delete an OIDC publisher from a channel
    DeleteOidc {
        /// Channel name
        channel: String,
        /// Publisher ID
        id: String,
    },
    /// Transfer channel ownership to another user
    Transfer {
        /// Channel name
        channel: String,
        /// New owner username
        new_owner: String,
    },
}

// ── Package Commands ────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum PackageCommand {
    /// Get package details
    Get {
        /// Channel name
        channel: String,
        /// Package name
        name: String,
        /// Variants page number
        #[arg(long, default_value = "0")]
        variants_page: i32,
        /// Variants per page
        #[arg(long, default_value = "25")]
        variants_limit: i32,
    },
    /// Search packages by name (similarity-based)
    Search {
        /// Search query
        query: String,
        /// Results per page
        #[arg(long, default_value = "25")]
        limit: i32,
        /// Page number (0-indexed)
        #[arg(long, default_value = "0")]
        page: i32,
    },
    /// List packages with filtering and pagination
    List {
        /// Filter by name (contains)
        #[arg(long)]
        name_contains: Option<String>,
        /// Results per page
        #[arg(long, default_value = "25")]
        limit: i32,
        /// Page number (0-indexed)
        #[arg(long, default_value = "0")]
        page: i32,
        /// Order by field
        #[arg(long, value_enum)]
        order_by: Option<PackageOrderField>,
        /// Sort direction
        #[arg(long, value_enum, default_value = "asc")]
        direction: SortDirection,
    },
    /// Find a package matching a matchspec
    Matchspec {
        /// Matchspec string
        spec: String,
        /// Channels to search (can be repeated)
        #[arg(long = "channel", required = true)]
        channels: Vec<String>,
    },
    /// Get a specific package variant
    Variant {
        /// Channel name
        channel: String,
        /// Package name
        package: String,
        /// Platform (e.g. linux-64)
        platform: String,
        /// Filename
        filename: String,
    },
    /// List package versions
    Versions {
        /// Channel name
        channel: String,
        /// Package name
        name: String,
        /// Results per page
        #[arg(long, default_value = "25")]
        limit: i32,
        /// Page number (0-indexed)
        #[arg(long, default_value = "0")]
        page: i32,
    },
    /// Yank a package variant
    Yank {
        /// Channel name
        channel: String,
        /// Platform subdir (e.g. linux-64)
        subdir: String,
        /// Package filename
        filename: String,
        /// Yank reason
        #[arg(long)]
        reason: String,
    },
    /// Unyank a package variant
    Unyank {
        /// Channel name
        channel: String,
        /// Platform subdir
        subdir: String,
        /// Package filename
        filename: String,
    },
    /// Batch delete package variants
    BatchDelete {
        /// Channel name
        channel: String,
        /// JSON array of entries: [{"subdir": "...", "filename": "..."}]
        #[arg(long)]
        entries: String,
    },
}

// ── Auth Commands ───────────────────────────────────────────────────────────

#[derive(Subcommand)]
pub enum AuthCommand {
    /// Show the currently authenticated user
    Whoami,
    /// Manage API keys
    ApiKey {
        #[command(subcommand)]
        command: ApiKeyCommand,
    },
}

#[derive(Subcommand)]
pub enum ApiKeyCommand {
    /// List all API keys
    List,
    /// Create a new API key
    Create {
        /// API key name
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Expiry datetime (RFC3339)
        #[arg(long)]
        expires_at: Option<String>,
    },
    /// Revoke an API key
    Revoke {
        /// API key name
        name: String,
    },
    /// Delete an API key
    Delete {
        /// API key name
        name: String,
    },
}

// ── Value Enums ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, ValueEnum)]
pub enum ChannelOrderField {
    Name,
    Size,
    CreatedAt,
    PackageCount,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum PackageOrderField {
    Name,
    LastCreatedDate,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum SortDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy, ValueEnum)]
pub enum MemberRole {
    Owner,
    Contributor,
    Viewer,
}
