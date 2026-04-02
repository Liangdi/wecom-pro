pub(crate) mod config;
pub(crate) mod error;

use anyhow::Result;

/// Look up the MCP URL for the given `category` (matched against `biz_type`) for a specific bot_id.
pub async fn get_mcp_url_by_id(bot_id: &str, category: &str) -> Result<String> {
    let Some(list) = config::load_mcp_config_by_id(bot_id) else {
        return Err(anyhow::anyhow!(
            "未找到 Bot '{}' 的 MCP 配置缓存，请先运行 `{} init --bot {}`",
            bot_id,
            env!("CARGO_BIN_NAME"),
            bot_id
        ));
    };

    let target = list
        .iter()
        .find(|item| item.biz_type.as_deref() == Some(category))
        .ok_or_else(|| anyhow::anyhow!("Bot '{}' 当前企业暂不支持 {} 命令", bot_id, category))?;

    target
        .url
        .clone()
        .ok_or_else(|| anyhow::anyhow!("Bot '{}' MCP 配置中 {} 的 url 为空", bot_id, category))
}

/// Look up the MCP URL for the given `category` (matched against `biz_type`).
/// This function maintains backward compatibility by defaulting to "default" bot.
#[allow(dead_code)]
pub async fn get_mcp_url(category: &str) -> Result<String> {
    get_mcp_url_by_id("default", category).await
}

/// Generate a request ID in the format: `{prefix}_{timestamp_ms}_{random_hex}`.
pub fn gen_req_id(prefix: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let random = generate_random_hex(8);
    format!("{prefix}_{timestamp}_{random}")
}

/// Generate a random hex string of the specified character length.
fn generate_random_hex(length: usize) -> String {
    let byte_len = length.div_ceil(2);
    let bytes: Vec<u8> = (0..byte_len).map(|_| rand::random()).collect();
    let hex = hex::encode(bytes);
    hex[..length].to_string()
}
