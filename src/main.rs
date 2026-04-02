mod auth;
mod cli_output;
mod cmd;
mod config;
mod constants;
mod crypto;
mod fs_util;
mod help;
mod json_rpc;
mod logging;
mod mcp;
mod media;

use anyhow::Result;
use clap::{Args, Command, Subcommand};

/// Entry point: parse CLI arguments and dispatch to the corresponding subcommand handler.
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    logging::init_logging();

    // 检查并迁移旧版本配置
    auth::check_and_migrate();

    let categories = config::get_categories();

    let mut cmd = Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .subcommand(cmd::init::InitArgs::augment_args(
            Command::new("init").about("初始化企业微信机器人配置"),
        ))
        .subcommand(cmd::bot::BotCommand::augment_subcommands(
            Command::new("bot").about("管理多个 Bot 配置"),
        ));

    for category in categories.iter() {
        cmd = cmd.subcommand(cmd::call::CallArgs::augment_args(
            Command::new(category.name)
                .about(category.description)
                .disable_help_subcommand(true)
                .disable_help_flag(true),
        ));
    }

    let matches = cmd.get_matches();

    let result: anyhow::Result<()> = match matches.subcommand() {
        Some(("init", matches)) => cmd::init::handle_init_cmd(matches).await,
        Some(("bot", matches)) => cmd::bot::handle_bot_cmd(matches).await,
        Some((category, matches)) => cmd::call::handle_call_cmd(category, matches).await,
        _ => anyhow::bail!("未知命令"),
    };

    match result {
        Err(e) => {
            cli_output::print_error(&e);
            std::process::exit(1);
        }
        Ok(_) => Ok(()),
    }
}
