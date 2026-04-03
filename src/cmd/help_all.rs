use crate::config;

/// Print comprehensive help for all commands
pub fn print_help_all() {
    let program_name = env!("CARGO_BIN_NAME");

    println!();
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!(
        "║                    {} 完整命令帮助                           ║",
        program_name.to_uppercase()
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    // Init command help
    print_init_help();
    println!();

    // Bot command help
    print_bot_help();
    println!();

    // Category commands help
    print_categories_help();
}

fn print_init_help() {
    let program_name = env!("CARGO_BIN_NAME");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  init - 初始化企业微信机器人配置");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("使用方式:");
    println!("    {} init [选项]", program_name);
    println!();
    println!("选项:");
    println!("  -b, --bot <id>           指定初始化的 Bot ID（默认为 default）");
    println!("  --method <method>        初始化方式：qrcode 或 manual（非交互模式）");
    println!("  --bot-id <id>            手动模式：企业微信机器人 Bot ID");
    println!("  --secret <secret>        手动模式：企业微信机器人 Secret");
    println!("  -o, --output <format>    输出格式：json 或 text（默认 text）");
    println!();
    println!("示例:");
    println!("    {} init", program_name);
    println!("    {} init --bot mybot", program_name);
    println!(
        "    {} init --method manual --bot-id xxx --secret yyy",
        program_name
    );
    println!("    {} init --method qrcode --output json", program_name);
}

fn print_bot_help() {
    let program_name = env!("CARGO_BIN_NAME");

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  bot - 管理多个 Bot 配置");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("子命令:");
    println!("    list                      列出所有已配置的 Bot");
    println!("    show <id>                 显示指定 Bot 的详细信息");
    println!("    remove <id>               删除指定 Bot");
    println!();
    println!("示例:");
    println!("    {} bot list", program_name);
    println!("    {} bot show mybot", program_name);
    println!("    {} bot remove mybot", program_name);
}

fn print_categories_help() {
    let program_name = env!("CARGO_BIN_NAME");
    let categories = config::get_categories();

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  业务命令分类");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    for category in categories {
        println!("{} - {}", category.name, category.description);
        println!(
            "    {} {} <method> [json_args] [选项]",
            program_name, category.name
        );
        println!("    {} {} --help", program_name, category.name);
        println!();
    }

    println!("通用选项:");
    println!("  -b, --bot <BOT>     指定使用的 Bot（默认使用 default）");
    println!("  -h, --help          显示该分类下所有工具的详细信息");
    println!();
    println!("示例:");
    println!("    {} msg list_chat", program_name);
    println!(
        "    {} msg send_text '{{\"to_chat_id\": \"xxx\", \"content\": \"hello\"}}'",
        program_name
    );
    println!("    {} schedule list --bot mybot", program_name);
}
