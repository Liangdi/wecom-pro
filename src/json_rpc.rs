use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

use crate::{constants, mcp};

#[derive(Debug, Clone, Serialize)]
struct JsonRpcRequest {
    jsonrpc: &'static str,
    id: String,
    method: String,
    params: Option<Value>,
}

/// Send a JSON-RPC 2.0 request to the MCP endpoint for the given category and method.
pub async fn send(
    category: &str,
    method: &str,
    params: Option<Value>,
    timeout_ms: Option<i32>,
) -> Result<Value> {
    let mcp_url = mcp::get_mcp_url(category).await?;

    let body = JsonRpcRequest {
        jsonrpc: "2.0",
        id: mcp::gen_req_id("mcp_rpc"),
        method: method.to_string(),
        params,
    };

    let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(30000) as u64);

    let request = reqwest::Client::builder()
        .build()?
        .post(&mcp_url)
        .timeout(timeout)
        .header("Accept", "application/json")
        .header("User-Agent", constants::get_user_agent())
        .json(&body);

    let response = request.send().await.map_err(|err| {
        if err.is_timeout() {
            anyhow::anyhow!("MCP请求超时 ({}ms)", timeout.as_millis())
        } else {
            anyhow::anyhow!("MCP网络请求失败: {err}")
        }
    })?;

    let status = response.status();

    if !status.is_success() {
        anyhow::bail!("MCP请求失败 (HTTP {status})");
    }

    let body_text = response.text().await?;
    let rpc_res = serde_json::from_str::<Value>(&body_text)?;

    Ok(rpc_res)
}
