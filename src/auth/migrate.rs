use std::fs;

/// 将旧版本的 bot.enc 迁移到 bots/default.enc
pub fn migrate_old_bot_to_default() -> bool {
    let config_dir = crate::constants::config_dir();
    let old_path = config_dir.join("bot.enc");
    let new_path = config_dir.join("bots").join("default.enc");

    if old_path.exists() && !new_path.exists() {
        if let Err(e) = fs::create_dir_all(new_path.parent().unwrap())
            .and_then(|_| fs::rename(&old_path, &new_path))
        {
            tracing::warn!("迁移 bot.enc 到 bots/default.enc 失败: {}", e);
            false
        } else {
            tracing::info!("已迁移 bot.enc 到 bots/default.enc");
            true
        }
    } else {
        false
    }
}

/// 将旧版本的 mcp_config.enc 迁移到 mcp_configs/default.enc
pub fn migrate_old_mcp_config_to_default() -> bool {
    let config_dir = crate::constants::config_dir();
    let old_path = config_dir.join("mcp_config.enc");
    let new_path = config_dir.join("mcp_configs").join("default.enc");

    if old_path.exists() && !new_path.exists() {
        if let Err(e) = fs::create_dir_all(new_path.parent().unwrap())
            .and_then(|_| fs::rename(&old_path, &new_path))
        {
            tracing::warn!("迁移 mcp_config.enc 到 mcp_configs/default.enc 失败: {}", e);
            false
        } else {
            tracing::info!("已迁移 mcp_config.enc 到 mcp_configs/default.enc");
            true
        }
    } else {
        false
    }
}

/// 检查是否需要迁移并执行迁移
pub fn check_and_migrate() {
    let bot_migrated = migrate_old_bot_to_default();
    let mcp_migrated = migrate_old_mcp_config_to_default();

    if bot_migrated || mcp_migrated {
        tracing::info!("配置迁移完成");
    }
}
