use std::time::Duration;

use anyhow::{Result, bail};
use serde::Deserialize;

use super::bot::Bot;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const SOURCE: &str = "wecom_cli_external";
const QR_GENERATE_URL: &str = "https://work.weixin.qq.com/ai/qc/generate";
const QR_QUERY_URL: &str = "https://work.weixin.qq.com/ai/qc/query_result";
const QR_CODE_PAGE: &str = "https://work.weixin.qq.com/ai/qc/gen";

/// 轮询间隔 3 秒
const POLL_INTERVAL: Duration = Duration::from_secs(3);
/// 超时 5 分钟
const POLL_TIMEOUT: Duration = Duration::from_secs(300);

// ---------------------------------------------------------------------------
// Response types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct GenerateResponse {
    data: Option<GenerateData>,
}

#[derive(Deserialize)]
struct GenerateData {
    scode: Option<String>,
    auth_url: Option<String>,
}

#[derive(Deserialize)]
struct QueryResponse {
    data: Option<QueryData>,
}

#[derive(Deserialize)]
struct QueryData {
    status: Option<String>,
    bot_info: Option<BotInfoPayload>,
}

#[derive(Deserialize)]
struct BotInfoPayload {
    botid: Option<String>,
    secret: Option<String>,
}

// ---------------------------------------------------------------------------
// Platform code
// ---------------------------------------------------------------------------

pub fn get_plat_code() -> u8 {
    if cfg!(target_os = "macos") {
        1
    } else if cfg!(target_os = "windows") {
        2
    } else if cfg!(target_os = "linux") {
        3
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// HTTP helpers
// ---------------------------------------------------------------------------

fn build_client() -> Result<reqwest::Client> {
    Ok(reqwest::Client::builder().build()?)
}

/// 获取二维码链接和轮询 scode
pub async fn fetch_qrcode(client: &reqwest::Client) -> Result<(String, String)> {
    let url = format!(
        "{}?source={}&plat={}",
        QR_GENERATE_URL,
        SOURCE,
        get_plat_code()
    );

    let resp: GenerateResponse = client.get(&url).send().await?.json().await?;

    let Some(data) = resp.data.as_ref() else {
        bail!("获取二维码失败，响应格式异常");
    };

    let (Some(scode), Some(auth_url)) = (&data.scode, &data.auth_url) else {
        bail!("获取二维码失败，响应格式异常");
    };

    Ok((scode.to_string(), auth_url.to_string()))
}

/// 在终端渲染二维码
fn render_qrcode(url: &str) -> Result<()> {
    println!();
    qr2term::print_qr(url).map_err(|e| anyhow::anyhow!("二维码渲染失败: {e}"))?;
    Ok(())
}

/// 轮询扫码结果
pub async fn poll_result(client: &reqwest::Client, scode: &str) -> Result<(String, String)> {
    let url = format!("{}?scode={}", QR_QUERY_URL, scode);
    let start = std::time::Instant::now();

    loop {
        if start.elapsed() >= POLL_TIMEOUT {
            bail!("扫码超时（5 分钟），请重试。");
        }

        let resp: QueryResponse = client.get(&url).send().await?.json().await?;

        if let Some(data) = &resp.data {
            if data.status.as_deref() == Some("success") {
                let Some(bot_info) = &data.bot_info else {
                    anyhow::bail!("扫码成功但未获取到 Bot 信息");
                };
                let (Some(botid), Some(secret)) = (&bot_info.botid, &bot_info.secret) else {
                    anyhow::bail!("扫码成功但未获取到 Bot 信息");
                };

                return Ok((botid.to_string(), secret.to_string()));
            }
        }

        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// 扫码接入完整流程：获取二维码 → 终端展示 → 轮询结果 → 返回 Bot
pub async fn scan_qrcode_for_bot() -> Result<Bot> {
    let client = build_client()?;

    println!("正在获取二维码...");
    let (scode, auth_url) = fetch_qrcode(&client).await?;

    println!("请使用企业微信扫描以下二维码：");
    render_qrcode(&auth_url)?;

    println!(
        "也可打开二维码链接扫码: {}?source={}&scode={}",
        QR_CODE_PAGE, SOURCE, scode
    );
    println!("等待扫码中...");

    let (bot_id, secret) = poll_result(&client, &scode).await?;

    println!("✔ 扫码成功！Bot ID 和 Secret 已自动获取。");

    Ok(Bot::new(bot_id, secret))
}
