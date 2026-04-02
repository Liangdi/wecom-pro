use crate::{config, json_rpc};
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static TOOLS_CACHE: RefCell<HashMap<String, Vec<serde_json::Value>>> = RefCell::new(HashMap::new());
}

/// Fetch and display the available tools for a given category via JSON-RPC.
#[allow(dead_code)]
pub async fn show_category_tools(category: &str) -> Result<()> {
    show_category_tools_with_bot(category, "default").await
}

/// Fetch and display the available tools for a given category via JSON-RPC with specific bot.
pub async fn show_category_tools_with_bot(category: &str, bot_id: &str) -> Result<()> {
    // Try to dynamically fetch the tool list via JSON-RPC
    let res = json_rpc::send_by_id(bot_id, category, "tools/list", None, None).await?;

    // Parse the returned tool list
    let Some(tools) = res
        .get("result")
        .and_then(|r| r.get("tools"))
        .and_then(|r| r.as_array())
    else {
        anyhow::bail!("无法获取 {} 品类的工具列表: {res}", category);
    };

    // Store to cache
    TOOLS_CACHE.with(|cache| {
        let mut cache_ref = cache.borrow_mut();
        cache_ref.insert(category.to_string(), tools.clone());
    });

    // Get category description
    let category_description = config::get_categories()
        .iter()
        .find(|info| info.name == category)
        .map(|info| info.description)
        .unwrap_or("");

    let wecom = env!("CARGO_BIN_NAME");

    println!("# {} {}", category, category_description);
    println!();
    println!("使用方式:");
    println!("    {} {} <method> [json_args]", wecom, category);
    println!();
    println!("选项:");
    println!("    -h, --help        显示详细的工具 schema 信息");
    println!();
    for tool in tools {
        let Some(name) = tool.get("name").and_then(|n| n.as_str()) else {
            continue;
        };
        println!();
        println!("## {}", name);
        if let Some(description) = tool.get("description").and_then(|d| d.as_str()) {
            println!();
            println!("{}", description);
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn show_tool_help(category: &str, tool_name: &str) -> Result<()> {
    show_tool_help_with_bot(category, tool_name, "default").await
}

pub async fn show_tool_help_with_bot(category: &str, tool_name: &str, bot_id: &str) -> Result<()> {
    // First try to get tool information from cache
    let tools = TOOLS_CACHE.with(|cache| {
        let cache_ref = cache.borrow();
        cache_ref.get(category).cloned()
    });

    let tools = if let Some(tools) = tools {
        tools
    } else {
        // If not in cache, fetch dynamically
        let res = json_rpc::send_by_id(bot_id, category, "tools/list", None, None).await?;

        let tools = res
            .get("result")
            .and_then(|r| r.get("tools"))
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        // Store to cache
        TOOLS_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            cache_ref.insert(category.to_string(), tools.clone());
        });

        tools
    };

    // Find specific tool
    let tool = tools.iter().find(|t| {
        t.get("name")
            .and_then(|n| n.as_str())
            .map(|name| name == tool_name)
            .unwrap_or(false)
    });

    if let Some(tool) = tool {
        println!("# {} - {}", category, tool_name);
        println!();

        if let Some(description) = tool.get("description").and_then(|d| d.as_str()) {
            println!("## 描述");
            println!("{}", description);
            println!();
        }

        if let Some(input_schema) = tool.get("inputSchema") {
            println!("## 输入参数");
            println!("```json");
            println!(
                "{}",
                serde_json::to_string_pretty(input_schema).unwrap_or_default()
            );
            println!("```");
            println!();
        }

        if let Some(parameters) = tool.get("parameters") {
            println!("## 参数");
            println!("```json");
            println!(
                "{}",
                serde_json::to_string_pretty(parameters).unwrap_or_default()
            );
            println!("```");
        }
    } else {
        println!("未找到工具: {} 在品类 {} 中", tool_name, category);
    }

    Ok(())
}
