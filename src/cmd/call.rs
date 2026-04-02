use crate::{config, help, json_rpc, media};

use anyhow::Result;
use clap::{ArgMatches, Args, FromArgMatches};
use serde_json::json;

#[derive(Args)]
pub struct CallArgs {
    /// 要调用的工具方法名
    #[arg(value_name = "method")]
    pub method: Option<String>,

    /// JSON 格式的参数
    #[arg(value_name = "args")]
    pub args: Option<String>,

    /// 指定使用的 Bot（默认使用 default）
    #[arg(long, short = 'b')]
    pub bot: Option<String>,

    #[arg(long, short)]
    pub help: bool,
}

/// Handle the `call` subcommand: dispatch a JSON-RPC tool invocation for a given category and method.
pub async fn handle_call_cmd(category_name: &str, matches: &ArgMatches) -> Result<()> {
    let args = CallArgs::from_arg_matches(matches)?;

    // Check if the category is valid
    let categories = config::get_categories();
    if !categories.iter().any(|c| c.name == category_name) {
        anyhow::bail!("无效命令：{}", category_name);
    }

    // Handle help first - don't require bot to exist for help
    if args.help {
        if let Some(method) = args.method.as_deref() {
            // For method-specific help, we need bot configuration
            // Show static help if bot doesn't exist
            let help_bot_id = args.bot.as_deref().unwrap_or("default");
            if crate::auth::get_bot_info_by_id(help_bot_id).is_some() {
                help::show_tool_help_with_bot(category_name, method, help_bot_id).await?;
            } else {
                // Show static help message
                println!("# {} - {}", category_name, method);
                println!();
                println!(
                    "未找到 Bot '{}' 的配置，无法显示详细的工具信息。",
                    help_bot_id
                );
                println!(
                    "请先运行: {} init --bot {}",
                    env!("CARGO_BIN_NAME"),
                    help_bot_id
                );
            }
        } else {
            // For category help, check if any bot exists
            let help_bot_id = args.bot.as_deref().unwrap_or("default");
            if crate::auth::get_bot_info_by_id(help_bot_id).is_some() {
                help::show_category_tools_with_bot(category_name, help_bot_id).await?;
            } else {
                // Show static category help
                let categories = config::get_categories();
                if let Some(cat) = categories.iter().find(|c| c.name == category_name) {
                    println!("# {} {}", category_name, cat.description);
                    println!();
                    println!("使用方式:");
                    println!(
                        "    {} {} <method> [json_args]",
                        env!("CARGO_BIN_NAME"),
                        category_name
                    );
                    println!();
                    println!("选项:");
                    println!("  -b, --bot <BOT>  指定使用的 Bot（默认使用 default）");
                    println!("  -h, --help        显示详细的工具 schema 信息");
                    println!();
                    println!(
                        "未找到 Bot '{}' 的配置，无法显示可用工具列表。",
                        help_bot_id
                    );
                    println!(
                        "请先运行: {} init --bot {}",
                        env!("CARGO_BIN_NAME"),
                        help_bot_id
                    );
                }
            }
        }
        return Ok(());
    }

    // Determine the bot_id to use
    let bot_id = args.bot.as_deref().unwrap_or("default");

    // Get positional arg: method
    let Some(method) = args.method.as_deref() else {
        // No method provided, show category tools list or helpful message
        // Check if bot exists first
        if crate::auth::get_bot_info_by_id(bot_id).is_none() {
            println!("未找到 Bot '{}' 的配置", bot_id);
            println!("请先运行以下命令初始化：");
            if bot_id == "default" {
                println!("  {} init", env!("CARGO_BIN_NAME"));
            } else {
                println!("  {} init --bot {}", env!("CARGO_BIN_NAME"), bot_id);
            }
            println!();
            println!("或者使用其他已配置的 Bot：");
            if let Ok(bots) = crate::auth::list_bots() {
                if bots.is_empty() {
                    println!("  (没有已配置的 Bot)");
                } else {
                    for bot in bots {
                        println!(
                            "  {} {} --bot {}",
                            env!("CARGO_BIN_NAME"),
                            category_name,
                            bot.id
                        );
                    }
                }
            }
            return Ok(());
        }
        help::show_category_tools_with_bot(category_name, bot_id).await?;
        return Ok(());
    };

    // Get positional arg: json_args (optional)
    let cli_args = args.args.as_deref();

    // If no arguments provided, show tool help information
    if cli_args.is_none() {
        help::show_tool_help_with_bot(category_name, method, bot_id).await?;
        return Ok(());
    };

    // Verify bot exists (only check if we have a method to execute)
    if crate::auth::get_bot_info_by_id(bot_id).is_none() {
        anyhow::bail!(
            "Bot '{}' 不存在，请先使用 `{} init --bot {}` 添加",
            bot_id,
            env!("CARGO_BIN_NAME"),
            bot_id
        );
    }

    // Get positional arg: json_args (optional)
    let cli_args = args.args.as_deref();

    // If no arguments provided, show tool help information
    if cli_args.is_none() {
        help::show_tool_help_with_bot(category_name, method, bot_id).await?;
        return Ok(());
    }

    let timeout_ms = if method == "get_msg_media" {
        Some(120000)
    } else {
        None
    };

    let parsed_args = if let Some(cli_args) = cli_args {
        serde_json::from_str(cli_args)?
    } else {
        json!({})
    };

    let params = json!({
        "name": method,
        "arguments": parsed_args,
    });

    let mut res = json_rpc::send_by_id(
        bot_id,
        category_name,
        "tools/call",
        Some(params),
        timeout_ms,
    )
    .await?;

    if method == "get_msg_media" {
        res = media::intercept_media_response(res).await?;
    }

    if let Some(result) = res.get("result") {
        println!("{}", result);
    }

    Ok(())
}
