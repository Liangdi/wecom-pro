// 多 Bot 使用示例
// 这是一个展示如何使用多 bot 功能的示例程序

fn main() -> anyhow::Result<()> {
    println!("=== 企业微信 CLI 多 Bot 使用示例 ===");
    println!();
    println!("本文档展示如何使用 wecom-cli 的多 Bot 功能。");
    println!();

    println!("## 基本概念");
    println!("- 单 Bot 模式: 不指定 --bot 参数，使用默认 'default' bot");
    println!("- 多 Bot 模式: 通过 --bot 参数指定使用的 bot");
    println!("- Bot 隔离: 每个 bot 有独立的配置和 MCP 缓存");
    println!();

    println!("## 使用场景");
    println!("1. 工作与个人分离 - 使用 work bot 处理工作，personal bot 处理个人事务");
    println!("2. 多项目管理 - 为不同项目配置专用 bot");
    println!("3. 团队协作 - 不同团队使用各自的 bot");
    println!("4. 环境隔离 - 开发、测试、生产环境使用不同的 bot");
    println!();

    println!("## 命令行使用示例");
    println!();

    println!("### 1. 初始化多个 Bot");
    println!("```bash");
    println!("# 初始化默认 bot");
    println!("wecom-cli init");
    println!();
    println!("# 初始化工作 bot");
    println!("wecom-cli init --bot work");
    println!();
    println!("# 初始化个人 bot");
    println!("wecom-cli init --bot personal");
    println!("```");
    println!();

    println!("### 2. 管理 Bot 配置");
    println!("```bash");
    println!("# 列出所有已配置的 Bot");
    println!("wecom-cli bot list");
    println!();
    println!("# 显示指定 Bot 的详细信息");
    println!("wecom-cli bot show work");
    println!();
    println!("# 删除指定 Bot（不能删除 default bot）");
    println!("wecom-cli bot remove project-a");
    println!("```");
    println!();

    println!("### 3. 使用指定 Bot 调用命令");
    println!("```bash");
    println!("# 使用 work bot 发送工作消息");
    println!("wecom-cli msg --bot work send_message \\");
    println!("  '{{\"chat_type\": 1, \"chatid\": \"zhangsan\"}}'");
    println!();

    println!("# 使用 personal bot 创建个人待办");
    println!("wecom-cli todo --bot personal create_todo \\");
    println!("  '{{\"content\": \"周末家庭聚餐\"}}'");
    println!("```");
    println!();

    println!("### 4. 向后兼容性");
    println!("```bash");
    println!("# 不指定 --bot 参数，使用默认 bot");
    println!("wecom-cli todo get_todo_list '{{}}'");
    println!();
    println!("# 等同于:");
    println!("wecom-cli todo --bot default get_todo_list '{{}}'");
    println!("```");
    println!();

    println!("## 实际应用场景");
    println!();

    println!("### 场景一：日常工作流");
    println!("```bash");
    println!("# 查看工作待办");
    println!("wecom-cli todo --bot work get_todo_list '{{}}'");
    println!();
    println!("# 创建工作待办");
    println!("wecom-cli todo --bot work create_todo \\");
    println!("  '{{\"content\": \"完成Q2报告\", \"todo_status\": 1}}'");
    println!();
    println!("# 发送工作通知");
    println!("wecom-cli msg --bot work send_message \\");
    println!("  '{{\"chat_type\": 1, \"chatid\": \"colleague\"}}'");
    println!("```");
    println!();

    println!("### 场景二：项目管理");
    println!("```bash");
    println!("# 为项目A创建文档");
    println!("wecom-cli doc --bot project-a create_doc \\");
    println!("  '{{\"doc_type\": 3, \"doc_name\": \"项目A需求文档\"}}'");
    println!();
    println!("# 创建项目会议");
    println!("wecom-cli meeting --bot project-a create_meeting \\");
    println!("  '{{\"title\": \"项目启动会话\"}}'");
    println!("```");
    println!();

    println!("## 配置存储结构");
    println!("```");
    println!("~/.config/wecom/");
    println!("├── bot.enc                  # default bot 配置");
    println!("├── bots/                   # 其他 bot 配置目录");
    println!("│   ├── work.enc");
    println!("│   ├── personal.enc");
    println!("│   └── project-a.enc");
    println!("└── mcp_configs/            # MCP 配置缓存");
    println!("    ├── default.enc");
    println!("    ├── work.enc");
    println!("    ├── personal.enc");
    println!("    └── project-a.enc");
    println!("```");
    println!();

    println!("## 注意事项");
    println!("1. 向后兼容: 不指定 --bot 参数时，默认使用 'default' bot");
    println!("2. 配置独立: 每个 bot 的待办、会议、文档相互独立");
    println!("3. 安全存储: 所有配置使用 AES-256-GCM 加密存储");
    println!("4. 权限管理: 默认 bot 不能删除，其他 bot 可以删除");
    println!("5. MCP 配置: 每个 bot 有独立的 MCP 配置缓存");
    println!();

    println!("## 相关文档");
    println!("- [完整使用指南](./multi_bot_guide.md)");
    println!("- [README](../README.md)");
    println!("- [API Skills](../skills/)");

    Ok(())
}
