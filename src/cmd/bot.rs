use crate::auth;
use anyhow::Result;
use clap::{ArgMatches, FromArgMatches, Subcommand};

#[derive(Subcommand)]
pub enum BotCommand {
    /// 列出所有已配置的 Bot
    List,
    /// 显示指定 Bot 的详细信息
    Show {
        /// Bot ID
        #[arg(value_name = "id")]
        id: String,
    },
    /// 删除指定 Bot
    Remove {
        /// Bot ID
        #[arg(value_name = "id")]
        id: String,
    },
}

pub async fn handle_bot_cmd(matches: &ArgMatches) -> Result<()> {
    let cmd = BotCommand::from_arg_matches(matches)?;

    match cmd {
        BotCommand::List => {
            let bots = auth::list_bots()?;
            if bots.is_empty() {
                println!("未找到任何已配置的 Bot");
                println!(
                    "请使用 `{} init --bot <bot_id>` 添加新的 Bot",
                    env!("CARGO_BIN_NAME")
                );
            } else {
                println!("已配置的 Bot：");
                for bot in &bots {
                    let create_time = format_timestamp(bot.create_time);
                    println!(
                        "  - {} (创建时间: {}){}",
                        bot.id,
                        create_time,
                        if bot.id == "default" { " [默认]" } else { "" }
                    );
                }
            }
        }
        BotCommand::Show { id } => {
            let bot = auth::get_bot_info_by_id(&id)
                .ok_or_else(|| anyhow::anyhow!("Bot '{}' 不存在", id))?;
            println!("Bot ID: {}", bot.id);
            println!("创建时间: {}", format_timestamp(bot.create_time));
            println!("状态: 已配置");
        }
        BotCommand::Remove { id } => {
            if id == "default" {
                anyhow::bail!("无法删除默认 Bot，请使用 init 命令重新初始化");
            }
            if auth::get_bot_info_by_id(&id).is_none() {
                anyhow::bail!("Bot '{}' 不存在", id);
            }
            auth::clear_bot_info_by_id(&id);
            crate::mcp::config::clear_mcp_config_by_id(&id);
            println!("Bot '{}' 已删除", id);
        }
    }

    Ok(())
}

fn format_timestamp(timestamp: u64) -> String {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    let bot_created = UNIX_EPOCH + Duration::from_secs(timestamp);
    let now = SystemTime::now();

    // 计算从 bot 创建到现在的时间差
    match now.duration_since(bot_created) {
        Ok(duration) => {
            let total_secs = duration.as_secs();
            let days = total_secs / 86400;
            let hours = (total_secs % 86400) / 3600;
            let minutes = (total_secs % 3600) / 60;

            if days > 365 {
                let years = days / 365;
                format!("大约 {} 年前", years)
            } else if days > 0 {
                format!("大约 {} 天前", days)
            } else if hours > 0 {
                format!("大约 {} 小时前", hours)
            } else if minutes > 0 {
                format!("大约 {} 分钟前", minutes)
            } else {
                "刚刚".to_string()
            }
        }
        Err(_) => {
            // 如果 bot 创建时间在未来（系统时间错误），显示时间戳
            format!("时间戳: {}", timestamp)
        }
    }
}
