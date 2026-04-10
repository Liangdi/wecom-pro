use std::path::PathBuf;

const DEFAULT_MCP_CONFIG_ENDPOINT: &str =
    "https://qyapi.weixin.qq.com/cgi-bin/aibot/cli/get_mcp_config";

pub mod env {
    /// Config directory, defaults to ~/.config/wecom
    pub const CONFIG_DIR: &str = "WECOM_CLI_CONFIG_DIR";

    /// Temp directory, defaults to std::env::temp_dir().join("wecom")
    pub const TMP_DIR: &str = "WECOM_CLI_TMP_DIR";

    /// Log level
    pub const LOG_LEVEL: &str = "WECOM_CLI_LOG_LEVEL";

    /// Log file directory path
    pub const LOG_FILE: &str = "WECOM_CLI_LOG_FILE";

    /// MCP config URL
    pub const MCP_CONFIG_ENDPOINT: &str = "WECOM_CLI_MCP_CONFIG_ENDPOINT";

    /// Output format (json or text)
    #[allow(dead_code)]
    pub const OUTPUT_FORMAT: &str = "WECOM_CLI_OUTPUT_FORMAT";
}

/// Return the configuration directory path (env override or `~/.config/wecom`).
pub fn config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var(env::CONFIG_DIR) {
        return PathBuf::from(dir);
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("wecom")
}

/// Return the media temp directory path (env override or `<tmp>/wecom/media`).
pub fn media_dir() -> PathBuf {
    if let Ok(dir) = std::env::var(env::TMP_DIR) {
        return PathBuf::from(dir).join("media");
    }
    std::env::temp_dir().join("wecom").join("media")
}

/// Return the MCP config endpoint URL (env override or the default WeCom API).
pub fn mcp_config_endpoint() -> String {
    std::env::var(env::MCP_CONFIG_ENDPOINT)
        .unwrap_or_else(|_| DEFAULT_MCP_CONFIG_ENDPOINT.to_string())
}

pub fn get_user_agent() -> String {
    format!(
        "WeComCLI/{} distribution/{} {}/{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("WECOM_CLI_DISTRIBUTION").unwrap_or_else(|| "unknown"),
        std::env::consts::OS,
        std::env::consts::ARCH,
    )
}
