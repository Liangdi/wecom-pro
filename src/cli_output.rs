use anyhow::Error;
use serde::Serialize;
use serde_json::json;
use std::env;

#[derive(Debug, Serialize)]
struct CliError {
    success: bool,
    error: ErrorDetail,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<String>,
}

/// Check if JSON output format is enabled
pub fn is_json_output() -> bool {
    env::var("WECOM_CLI_OUTPUT_FORMAT")
        .map(|v| v.to_lowercase() == "json")
        .unwrap_or(false)
}

/// Print error in the appropriate format (JSON or text)
pub fn print_error(error: &Error) {
    if is_json_output() {
        print_error_json(error);
    } else {
        // Default text output - let anyhow handle it
        eprintln!("Error: {}", error);
    }
}

/// Print error in JSON format
fn print_error_json(error: &Error) {
    let error_detail = ErrorDetail {
        code: error_code_from_error(error),
        message: error_message_from_error(error),
        context: error.chain().nth(1).map(|e| e.to_string()),
    };

    let cli_error = CliError {
        success: false,
        error: error_detail,
    };

    if let Ok(json) = serde_json::to_string_pretty(&cli_error) {
        eprintln!("{}", json);
    } else {
        // Fallback to simple error if serialization fails
        eprintln!(
            "{}",
            json!({ "success": false, "error": { "code": "UNKNOWN", "message": error.to_string() } })
        );
    }
}

/// Print success message in the appropriate format
#[allow(dead_code)]
pub fn print_success(message: &str) {
    if is_json_output() {
        let success = json!({ "success": true, "message": message });
        eprintln!(
            "{}",
            serde_json::to_string_pretty(&success).unwrap_or_default()
        );
    } else {
        eprintln!("{}", message);
    }
}

/// Extract error code from anyhow::Error
fn error_code_from_error(error: &Error) -> String {
    let error_msg = error.to_string().to_lowercase();

    if error_msg.contains("bot") && error_msg.contains("已存在") {
        "BOT_ALREADY_EXISTS".to_string()
    } else if error_msg.contains("bot") && error_msg.contains("不存在") {
        "BOT_NOT_FOUND".to_string()
    } else if error_msg.contains("验证") || error_msg.contains("凭证") {
        "AUTH_FAILED".to_string()
    } else if error_msg.contains("网络") || error_msg.contains("http") {
        "NETWORK_ERROR".to_string()
    } else if error_msg.contains("超时") {
        "TIMEOUT".to_string()
    } else {
        "UNKNOWN_ERROR".to_string()
    }
}

/// Extract user-friendly error message
fn error_message_from_error(error: &Error) -> String {
    error.to_string()
}
