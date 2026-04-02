use crate::auth;
use crate::mcp;
use crate::mcp::config::McpBindSource;
use anyhow::Result;
use clap::{ArgMatches, Args, FromArgMatches};

#[derive(Args)]
pub struct InitArgs {
    /// 指定初始化的 Bot ID（默认为 default）
    #[arg(long, short = 'b')]
    pub bot: Option<String>,

    /// 初始化方式：qrcode 或 manual（非交互模式）
    #[arg(long, value_name = "method")]
    pub method: Option<String>,

    /// 手动模式：企业微信机器人 Bot ID
    #[arg(long, value_name = "id")]
    pub bot_id: Option<String>,

    /// 手动模式：企业微信机器人 Secret
    #[arg(long, value_name = "secret")]
    pub secret: Option<String>,

    /// 输出格式：json 或 text（默认 text）
    #[arg(long, short = 'o', value_name = "format")]
    pub output: Option<String>,
}

/// Check if JSON output format is enabled
fn is_json_output(args: &InitArgs) -> bool {
    args.output.as_deref().unwrap_or("text") == "json"
}

/// Validate command line arguments
fn validate_args(args: &InitArgs) -> Result<()> {
    match args.method.as_deref() {
        None => {
            // Interactive mode: bot_id and secret not allowed
            if args.bot_id.is_some() || args.secret.is_some() {
                anyhow::bail!("交互模式下不支持 --bot-id 和 --secret 参数");
            }
        }
        Some("qrcode") => {
            // QR code mode: bot_id and secret not allowed
            if args.bot_id.is_some() || args.secret.is_some() {
                anyhow::bail!("扫码模式下不支持 --bot-id 和 --secret 参数");
            }
        }
        Some("manual") => {
            // Manual mode: both bot_id and secret required
            if args.bot_id.is_none() || args.secret.is_none() {
                anyhow::bail!("手动模式需要同时提供 --bot-id 和 --secret 参数");
            }
        }
        Some(_) => {
            anyhow::bail!("无效的 --method 参数，支持：qrcode, manual");
        }
    }
    Ok(())
}

/// Get platform code for QR code generation
fn get_plat_code() -> u8 {
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

/// Handle bot already exists error
fn handle_bot_exists(bot_id: &str, json_output: bool) -> Result<()> {
    let error_msg = format!(
        "Bot ID \"{}\" 已存在，如需重新初始化请先使用 `{} bot remove {}` 清除",
        bot_id,
        env!("CARGO_BIN_NAME"),
        bot_id
    );
    if json_output {
        eprintln!(
            r#"{{"success": false, "error": {{"code": "BOT_EXISTS", "message": "Bot ID \"{}\" 已存在"}}}}"#,
            bot_id
        );
        anyhow::bail!("Bot ID \"{}\" 已存在", bot_id);
    } else {
        cliclack::outro(&error_msg)?;
        anyhow::bail!("Bot ID \"{}\" 已存在", bot_id);
    }
}

/// Handle the `init` subcommand: prompt for bot credentials, persist them, and verify via MCP config fetch.
pub async fn handle_init_cmd(matches: &ArgMatches) -> Result<()> {
    let args = InitArgs::from_arg_matches(matches)?;
    validate_args(&args)?;

    let bot_id = args.bot.as_deref().unwrap_or("default");
    let json_output = is_json_output(&args);

    // Check if bot already exists
    if auth::get_bot_info_by_id(bot_id).is_some() {
        return handle_bot_exists(bot_id, json_output);
    }

    // Route to appropriate mode
    match args.method.as_deref() {
        None => handle_init_interactive(bot_id, json_output).await,
        Some(method) => handle_init_non_interactive(bot_id, method, &args).await,
    }
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

/// Handle interactive mode initialization
async fn handle_init_interactive(bot_id: &str, _json_output: bool) -> Result<()> {
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
    verify_and_finish(bot_id, bind_source, false).await
}

/// Handle non-interactive mode initialization
async fn handle_init_non_interactive(bot_id: &str, method: &str, args: &InitArgs) -> Result<()> {
    let json_output = is_json_output(args);

    match method {
        "manual" => {
            // Manual mode: use provided credentials
            let bot = auth::Bot::new(
                args.bot_id.as_ref().unwrap().clone(),
                args.secret.as_ref().unwrap().clone(),
            );
            auth::set_bot_info_by_id(bot_id, &bot)?;

            if json_output {
                println!(
                    r#"{{"success": true, "bot_id": "{}", "message": "初始化成功"}}"#,
                    bot_id
                );
            } else {
                println!("初始化完成 ✅");
            }

            verify_and_finish(bot_id, McpBindSource::Manual, json_output).await
        }
        "qrcode" => {
            // QR code mode: generate QR code URL and output
            let client = reqwest::Client::new();
            let (scode, _auth_url) = auth::fetch_qrcode(&client).await?;

            let qrcode_url = format!(
                "https://work.weixin.qq.com/ai/qc/gen?source=wecom_cli_external&plat={}&scode={}",
                get_plat_code(),
                scode
            );

            if json_output {
                println!(
                    r#"{{"success": true, "qrcode_url": "{}", "polling": true}}"#,
                    qrcode_url
                );
            } else {
                println!("请打开以下链接扫码：");
                println!("{}", qrcode_url);
                println!("等待扫码中...");
            }

            // Poll for scan result
            let (bot_id_result, secret) = auth::poll_result(&client, &scode).await?;
            let bot = auth::Bot::new(bot_id_result, secret);
            auth::set_bot_info_by_id(bot_id, &bot)?;

            verify_and_finish(bot_id, McpBindSource::Qrcode, json_output).await
        }
        _ => unreachable!(),
    }
}

/// 验证凭证并完成初始化
async fn verify_and_finish(
    bot_id: &str,
    bind_source: McpBindSource,
    json_output: bool,
) -> Result<()> {
    let spinner = if json_output {
        None
    } else {
        Some(cliclack::spinner())
    };

    if let Some(ref s) = spinner {
        s.start("正在验证企业微信机器人凭证...");
    }

    if let Err(e) = mcp::config::fetch_mcp_config_by_id(bot_id, bind_source).await {
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

        if json_output {
            eprintln!(
                r#"{{"success": false, "error": {{"code": "AUTH_FAILED", "message": "{}"}}}}"#,
                output_errmsg
            );
        } else {
            if let Some(s) = spinner {
                s.stop("企业微信机器人凭证验证失败");
            }
            cliclack::outro("初始化失败 ❌")?;
        }
        anyhow::bail!(output_errmsg);
    }

    if json_output {
        println!(
            r#"{{"success": true, "bot_id": "{}", "message": "初始化成功"}}"#,
            bot_id
        );
    } else {
        if let Some(s) = spinner {
            s.stop("企业微信机器人凭证验证成功");
        }
        cliclack::outro("初始化完成 ✅")?;
    }
    Ok(())
}
