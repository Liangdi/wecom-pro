use crate::auth;
use crate::cli_output;
use crate::mcp;
use crate::mcp::config::McpBindSource;
use anyhow::Result;
use clap::{ArgMatches, Args, FromArgMatches};

#[derive(Args)]
pub struct InitArgs {
    /// 指定初始化的 Bot ID（默认为 default）
    #[arg(long, short = 'b')]
    pub bot: Option<String>,
}

/// Handle the `init` subcommand: prompt for bot credentials, persist them, and verify via MCP config fetch.
pub async fn handle_init_cmd(matches: &ArgMatches) -> Result<()> {
    let args = InitArgs::from_arg_matches(matches)?;
    let bot_id = args.bot.as_deref().unwrap_or("default");

    // 检查 bot_id 是否已被初始化
    if auth::get_bot_info_by_id(bot_id).is_some() {
        let error_msg = format!(
            "Bot ID \"{}\" 已存在，如需重新初始化请先使用 `{} bot remove {}` 清除",
            bot_id,
            env!("CARGO_BIN_NAME"),
            bot_id
        );
        if cli_output::is_json_output() {
            anyhow::bail!("Bot ID \"{}\" 已存在", bot_id);
        } else {
            cliclack::outro(&error_msg)?;
            anyhow::bail!("Bot ID \"{}\" 已存在", bot_id);
        }
    }

    cliclack::intro(format!("企业微信机器人初始化 - {}", bot_id))?;

    // 交互选择接入方式
    let method: &str = cliclack::select("请选择企微机器人接入方式：")
        .item("qrcode", "扫码接入（推荐）", "")
        .item("manual", "手动输入 Bot ID 和 Secret", "")
        .interact()?;

    let (bot, bind_source) = match method {
        "qrcode" => (init_qrcode().await?, McpBindSource::Qrcode),
        _ => (init_manual().await?, McpBindSource::Interactive),
    };

    auth::set_bot_info_by_id(bot_id, &bot)?;
    verify_and_finish(bot_id, bind_source).await
}

/// 扫码接入流程
async fn init_qrcode() -> Result<auth::Bot> {
    auth::scan_qrcode_for_bot().await
}

/// 手动输入 Bot ID 和 Secret
async fn init_manual() -> Result<auth::Bot> {
    let bot_id: String = cliclack::input("企业微信机器人 Bot ID")
        .placeholder("请输入企业微信机器人ID")
        .interact()?;

    let bot_secret: String = cliclack::password("企业微信机器人 Secret")
        .mask('*')
        .interact()?;

    Ok(auth::Bot::new(bot_id, bot_secret))
}

/// 验证凭证并完成初始化
async fn verify_and_finish(bot_id: &str, bind_source: McpBindSource) -> Result<()> {
    let spinner = cliclack::spinner();
    spinner.start("正在验证企业微信机器人凭证...");

    if let Err(e) = mcp::config::fetch_mcp_config_by_id(bot_id, bind_source).await {
        spinner.stop("企业微信机器人凭证验证失败");

        let mut output_errmsg: String = "验证企业微信机器人凭证失败".to_owned();

        match &e {
            mcp::error::FetchMcpConfigError::Api(resp) => {
                if let Some(ref msg) = resp.errmsg {
                    if !msg.is_empty() {
                        output_errmsg = msg.clone();
                    }
                }
            }
            mcp::error::FetchMcpConfigError::Http(http_err) => {
                output_errmsg = format!("{} HTTP返回状态码 {}", output_errmsg, http_err.status);
            }
            mcp::error::FetchMcpConfigError::Other(other_err) => {
                output_errmsg = other_err.to_string();
            }
        }

        // Credentials invalid or server unreachable — rollback
        auth::clear_bot_info_by_id(bot_id);
        mcp::config::clear_mcp_config_by_id(bot_id);
        cliclack::outro("初始化失败 ❌")?;
        anyhow::bail!(output_errmsg);
    }

    spinner.stop("企业微信机器人凭证验证成功");
    cliclack::outro("初始化完成 ✅")?;
    Ok(())
}
