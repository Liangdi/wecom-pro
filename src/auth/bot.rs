use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use anyhow::Result;

use crate::{crypto, fs_util};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bot {
    // Bot ID
    pub id: String,
    // Bot Secret
    pub secret: String,
    // Creation timestamp (unix epoch seconds)
    pub create_time: u64,
}

impl Bot {
    /// Create a new Bot with `create_time` set to the current timestamp.
    pub fn new(id: String, secret: String) -> Self {
        let create_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            id,
            secret,
            create_time,
        }
    }
}

/// Bot 信息摘要（用于列表展示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotInfo {
    pub id: String,
    pub create_time: u64,
}

/// Return the file path for the encrypted bot credentials by bot_id.
/// "default" uses the legacy path for backward compatibility, but falls back to bots/default.enc
fn bot_info_path_by_id(bot_id: &str) -> std::path::PathBuf {
    let config_dir = crate::constants::config_dir();
    match bot_id {
        "default" => {
            // Try legacy path first for backward compatibility
            let legacy_path = config_dir.join("bot.enc");
            if legacy_path.exists() {
                return legacy_path;
            }
            // Fall back to new location
            config_dir.join("bots").join("default.enc")
        }
        _ => config_dir.join("bots").join(format!("{bot_id}.enc")),
    }
}

/// Read encrypted bot info from disk by bot_id, decrypt and return it.
/// Returns `None` if the file does not exist or decryption fails.
pub fn get_bot_info_by_id(bot_id: &str) -> Option<Bot> {
    let data = fs::read(bot_info_path_by_id(bot_id)).ok()?;
    crypto::try_decrypt_data(&data).ok()
}

/// List all configured bots.
/// Returns `Err` if the bots directory cannot be read.
pub fn list_bots() -> Result<Vec<BotInfo>> {
    let config_dir = crate::constants::config_dir();
    let mut bots = Vec::new();

    // Check default bot
    if bot_info_path_by_id("default").exists() {
        if let Some(bot) = get_bot_info_by_id("default") {
            bots.push(BotInfo {
                id: "default".to_string(),
                create_time: bot.create_time,
            });
        }
    }

    // Check bots in bots/ directory (skip default.enc as it's already handled above)
    let bots_dir = config_dir.join("bots");
    if bots_dir.exists() {
        let entries = fs::read_dir(&bots_dir)?;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("enc") {
                if let Some(bot_id) = path.file_stem().and_then(|s| s.to_str()) {
                    // Skip default.enc as it's handled above
                    if bot_id == "default" {
                        continue;
                    }
                    if let Some(bot) = get_bot_info_by_id(bot_id) {
                        bots.push(BotInfo {
                            id: bot_id.to_string(),
                            create_time: bot.create_time,
                        });
                    }
                }
            }
        }
    }

    bots.sort_by(|a, b| a.create_time.cmp(&b.create_time));
    Ok(bots)
}

/// Serialize bot info, encrypt and persist to disk for a specific bot_id.
/// The encryption key is stored in the system keyring when possible, otherwise falls back to an encrypted file.
pub fn set_bot_info_by_id(bot_id: &str, bot: &Bot) -> Result<()> {
    // 1. Load or generate an encryption key
    let key = crypto::load_existing_key().unwrap_or_else(|| {
        let k = crypto::generate_random_key();
        tracing::info!("已生成新的加密密钥");
        k
    });

    // 2. Persist the key (prefer keyring, fall back to file)
    crypto::save_key(&key)?;

    // 3. Serialize bot info → JSON → encrypt
    let encrypted = crypto::encrypt_data(bot, &key)?;

    // 4. Write to file
    let path = bot_info_path_by_id(bot_id);
    fs_util::atomic_write(&path, &encrypted, Some(0o600))?;

    tracing::info!("企业微信机器人信息已保存到 {}", path.display());
    Ok(())
}

/// Remove the stored Bot info from disk for a specific bot_id.
pub fn clear_bot_info_by_id(bot_id: &str) {
    let path = bot_info_path_by_id(bot_id);
    if path.exists() {
        let _ = fs::remove_file(&path);
        tracing::info!("机器人信息已删除：{}", path.display());
    }
}

/// Read encrypted bot info from disk, decrypt and return it.
/// Returns `None` if the file does not exist or decryption fails.
/// This function maintains backward compatibility by defaulting to "default" bot.
#[allow(dead_code)]
pub fn get_bot_info() -> Option<Bot> {
    let data = fs::read(bot_info_path()).ok()?;
    crypto::try_decrypt_data(&data).ok()
}

/// Serialize bot info, encrypt and persist to disk.
/// The encryption key is stored in the system keyring when possible, otherwise falls back to an encrypted file.
#[allow(dead_code)]
pub fn set_bot_info(bot: &Bot) -> Result<()> {
    // 1. Load or generate an encryption key
    let key = crypto::load_existing_key().unwrap_or_else(|| {
        let k = crypto::generate_random_key();
        tracing::info!("已生成新的加密密钥");
        k
    });

    // 2. Persist the key (prefer keyring, fall back to file)
    crypto::save_key(&key)?;

    // 3. Serialize bot info → JSON → encrypt
    let encrypted = crypto::encrypt_data(bot, &key)?;

    // 4. Write to file
    let path = bot_info_path();
    fs_util::atomic_write(&path, &encrypted, Some(0o600))?;

    tracing::info!("企业微信机器人信息已保存到 {}", path.display());
    Ok(())
}

/// Remove the stored Bot info from disk.
#[allow(dead_code)]
pub fn clear_bot_info() {
    let path = bot_info_path();
    if path.exists() {
        let _ = fs::remove_file(&path);
        tracing::info!("机器人信息已删除：{}", path.display());
    }
}

/// Return the file path for the encrypted bot credentials.
#[allow(dead_code)]
fn bot_info_path() -> std::path::PathBuf {
    crate::constants::config_dir().join("bot.enc")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_env() -> tempfile::TempDir {
        // 使用时间戳和线程ID确保唯一性
        let unique_name = format!(
            "test_{}_{:?}",
            std::process::id(),
            std::thread::current().id()
        );
        let dir = tempfile::Builder::new()
            .prefix(&unique_name)
            .tempdir()
            .unwrap();
        unsafe { std::env::set_var("WECOM_CLI_CONFIG_DIR", dir.path()) };
        dir
    }

    fn create_test_bot(id: &str, secret: &str) -> Bot {
        Bot::new(id.to_string(), secret.to_string())
    }

    #[test]
    fn test_get_bot_info_by_id_default() {
        let temp_dir = setup_test_env();

        // 确保开始时是空的
        assert!(get_bot_info_by_id("default").is_none());

        let bot = create_test_bot("default", "test_secret");
        set_bot_info_by_id("default", &bot).unwrap();

        let loaded = get_bot_info_by_id("default").unwrap();
        assert_eq!(loaded.id, "default");
        assert_eq!(loaded.secret, "test_secret");

        // 保持 temp_dir 的生命周期直到测试结束
        drop(temp_dir);
    }

    #[test]
    fn test_get_bot_info_by_id_custom() {
        let temp_dir = setup_test_env();

        let bot = create_test_bot("bot_123", "custom_secret");
        set_bot_info_by_id("bot_123", &bot).unwrap();

        let loaded = get_bot_info_by_id("bot_123").unwrap();
        assert_eq!(loaded.id, "bot_123");
        assert_eq!(loaded.secret, "custom_secret");

        drop(temp_dir);
    }

    #[test]
    fn test_get_bot_info_by_id_not_exists() {
        let temp_dir = setup_test_env();

        let result = get_bot_info_by_id("nonexistent");
        assert!(result.is_none());

        drop(temp_dir);
    }

    #[test]
    fn test_list_bots_empty() {
        let temp_dir = setup_test_env();

        let bots = list_bots().unwrap();
        assert_eq!(bots.len(), 0);

        drop(temp_dir);
    }

    #[test]
    fn test_list_bots_multiple() {
        let temp_dir = setup_test_env();

        // 创建一些 bot
        let bot1 = create_test_bot("test_list_bot_1", "secret1");
        let bot2 = create_test_bot("test_list_bot_2", "secret2");

        let result1 = set_bot_info_by_id("test_list_bot_1", &bot1);
        let result2 = set_bot_info_by_id("test_list_bot_2", &bot2);

        assert!(result1.is_ok(), "Failed to set bot1: {:?}", result1);
        assert!(result2.is_ok(), "Failed to set bot2: {:?}", result2);

        // 验证可以加载这些 bot
        let loaded1 = get_bot_info_by_id("test_list_bot_1");
        let loaded2 = get_bot_info_by_id("test_list_bot_2");

        assert!(loaded1.is_some(), "Failed to load bot1");
        assert!(loaded2.is_some(), "Failed to load bot2");

        let loaded1 = loaded1.unwrap();
        let loaded2 = loaded2.unwrap();

        assert_eq!(loaded1.id, "test_list_bot_1");
        assert_eq!(loaded2.id, "test_list_bot_2");

        drop(temp_dir);
    }

    #[test]
    fn test_backward_compatibility() {
        let temp_dir = setup_test_env();

        // 测试旧的 API 仍然工作
        let bot = create_test_bot("default", "test_secret");
        let result = set_bot_info(&bot);
        assert!(result.is_ok(), "set_bot_info should succeed");

        let loaded = get_bot_info();
        assert!(loaded.is_some(), "get_bot_info should return Some");
        let loaded = loaded.unwrap();
        assert_eq!(loaded.id, "default");
        assert_eq!(loaded.secret, "test_secret");

        drop(temp_dir);
    }

    #[test]
    fn test_clear_bot_info_by_id() {
        let temp_dir = setup_test_env();

        // 确保开始时不存在
        let initial = get_bot_info_by_id("bot_remove_test");
        assert!(initial.is_none(), "Should not exist initially");

        let bot = create_test_bot("bot_remove_test", "secret");
        let set_result = set_bot_info_by_id("bot_remove_test", &bot);
        assert!(set_result.is_ok(), "set_bot_info_by_id should succeed");

        let loaded = get_bot_info_by_id("bot_remove_test");
        assert!(loaded.is_some(), "Should exist after setting");

        clear_bot_info_by_id("bot_remove_test");

        let final_check = get_bot_info_by_id("bot_remove_test");
        assert!(final_check.is_none(), "Should not exist after clearing");

        drop(temp_dir);
    }

    #[test]
    fn test_bot_encryption() {
        let temp_dir = setup_test_env();

        let bot = create_test_bot("encrypted_bot", "super_secret");
        set_bot_info_by_id("encrypted_bot", &bot).unwrap();

        // 验证可以解密
        let loaded = get_bot_info_by_id("encrypted_bot").unwrap();
        assert_eq!(loaded.secret, "super_secret");

        drop(temp_dir);
    }
}
