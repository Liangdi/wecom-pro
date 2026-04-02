# wecom-pro

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%3E%3D1.75-orange.svg)](https://www.rust-lang.org/)

> 💬 扫码加入企业微信交流群：
>
> <img src="https://wwcdn.weixin.qq.com/node/wework/images/202603241759.3fb01c32cc.png" alt="扫码入群交流" width="200" />

企业微信命令行工具 — 让人类和 AI Agent 都能在终端中操作企业微信。覆盖通讯录、待办、会议、消息、日程、文档、智能表格等核心业务域，提供 7 大品类及 12 个 AI Agent [Skills](https://github.com/WecomTeam/wecom-pro/tree/main/skills)。

> **项目说明**: 本项目是从 [WecomTeam/wecom-pro](https://github.com/WecomTeam/wecom-pro) fork 而来，增加了**多 Bot 支持**功能。
> - **作者**: Liangdi <wu@liangdi.me>
> - **项目地址**: [https://github.com/Liangdi/wecom-pro](https://github.com/Liangdi/wecom-pro)

[安装](#安装与快速开始) · [AI Agent Skills](#agent-skills) · [命令](#命令参考) · [品类一览](#品类与能力一览)

## 为什么选 wecom-pro？

- **为 AI Agent 所设计** — 开箱即用的 [Skills](https://github.com/WecomTeam/wecom-cli/tree/main/skills)， 适配主流 AI 工具，Agent 可直接操作企业微信，无需额外适配
- **覆盖用户核心需求** — 7 大业务品类、12 个 AI Agent [Skills](https://github.com/WecomTeam/wecom-cli/tree/main/skills)，覆盖通讯录、待办、会议、消息、日程、文档与智能表格
- **多 Bot 支持** — 支持配置多个企业微信机器人，实现工作与个人账号分离，不同业务场景使用不同 Bot
- **快速上手** — `init` 配置凭证，直接调用品类工具，从安装到第一次 API 调用只需两步

## 功能

| 类别         | 能力                                                                          |
| ------------ | ----------------------------------------------------------------------------- |
| 👤 通讯录   | 获取可见范围成员列表、按姓名/别名搜索                                        |
| ✅ 待办     | 创建、查询列表、查询详情、更新、删除待办，变更用户处理状态                    |
| 🎥 会议     | 创建预约会议、取消会议、更新受邀成员、查询会议列表、获取会议详情              |
| 💬 消息     | 会话列表查询、消息记录拉取（文本/图片/文件/语音/视频）、多媒体下载、发送文本  |
| 📅 日程     | 日程 CRUD、参与人管理、多成员闲忙查询                                         |
| 📄 文档     | 文档创建/读取/编辑                                                            |
| 📊 智能表格   | 智能表格创建、子表与字段管理、表格记录增删改查                                  |

## 安装与快速开始

### 环境要求

- ~~Node.js（`npm`/`npx`）~~
- 企业微信机器人的 Bot ID 和 Secret

### 安装

~~```bash
# 安装 CLI
npm install -g @wecom/cli

# 安装 CLI SKILL（必需）
npx skills add WeComTeam/wecom-pro -y -g
```~~

> **注意**: 本项目为 Rust 实现，暂不提供 Node.js 版本。请从 [GitHub Releases](https://github.com/Liangdi/wecom-pro/releases) 下载预编译二进制文件，或使用 cargo 安装。

```bash
# 使用 cargo 安装（推荐）
cargo install wecom-pro

# 或从 GitHub Releases 下载
# Linux
wget https://github.com/Liangdi/wecom-pro/releases/download/v0.1.0/wecom-pro-linux-x64 -O wecom-pro
chmod +x wecom-pro

# macOS
wget https://github.com/Liangdi/wecom-pro/releases/download/v0.1.0/wecom-pro-macos-x64 -O wecom-pro
chmod +x wecom-pro

# 或从源码编译
git clone https://github.com/Liangdi/wecom-pro.git
cd wecom-pro
cargo install --path .
```

### 快速开始

#### 单 Bot 模式（默认）

```bash
# 1. 配置企业微信机器人凭证（交互式，仅需一次）
wecom-pro init

# 2. 调用工具
wecom-pro contact get_userlist '{}'
```

#### 多 Bot 模式

```bash
# 1. 初始化多个 Bot
wecom-pro init --bot work
wecom-pro init --bot personal

# 2. 查看已配置的 Bot
wecom-pro bot list

# 3. 使用指定 Bot 调用工具
wecom-pro msg --bot work send_message '{"chat_type": 1, "chatid": "zhangsan", "msgtype": "text", "text": {"content": "工作消息"}}'
wecom-pro todo --bot personal create_todo '{"content": "个人待办", "todo_status": 1}'
```

## Agent Skills

| Skill | 品类 | 说明 |
| ----- | ---- | ---- |
| `wecomcli-lookup-contact` | contact | 通讯录成员查询，按姓名/别名搜索 |
| `wecomcli-get-todo-list` | todo | 待办列表查询，按时间过滤和分页 |
| `wecomcli-get-todo-detail` | todo | 待办详情批量查询 |
| `wecomcli-edit-todo` | todo | 待办创建、更新、删除、状态变更 |
| `wecomcli-create-meeting` | meeting | 创建预约会议 |
| `wecomcli-edit-meeting` | meeting | 取消会议、更新受邀成员 |
| `wecomcli-get-meeting` | meeting | 查询会议列表和详情 |
| `wecomcli-get-msg` | msg | 会话列表、消息记录、媒体下载、文本发送 |
| `wecomcli-manage-schedule` | schedule | 日程 CRUD、参与人管理、闲忙查询 |
| `wecomcli-manage-doc` | doc | 文档创建/读取/编辑 |
| `wecomcli-manage-smartsheet-schema` | smartsheet | 智能表格子表与字段管理 |
| `wecomcli-manage-smartsheet-data` | smartsheet | 智能表格记录增删改查 |

## 命令参考

### `--help`

列出所有支持的命令和品类。

```bash
wecom-pro --help
```

输出示例：

```
Usage: wecom-pro <COMMAND>

Commands:
  init      初始化企业微信机器人配置
  bot       管理多个 Bot 配置
  contact   通讯录 — 成员查询和搜索
  doc       文档 — 文档/智能表格创建和管理
  meeting   会议 — 创建/管理/查询视频会议
  msg       消息 — 聊天列表、发送/接收消息、媒体下载
  schedule  日程 — 日程增删改查和可用性查询
  todo      待办事项 — 创建/查询/编辑待办项

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `init`

交互式配置企业微信机器人凭证，加密存储到本地。

```bash
# 初始化默认 Bot（单 Bot 模式）
wecom-pro init

# 初始化指定 Bot（多 Bot 模式）
wecom-pro init --bot work
wecom-pro init --bot personal
```

凭证存储位置：
- 默认 Bot: `~/.config/wecom/bot.enc`
- 指定 Bot: `~/.config/wecom/bots/{bot_id}.enc`

### `bot`

管理多个 Bot 配置。

```bash
# 列出所有已配置的 Bot
wecom-pro bot list

# 显示指定 Bot 的详细信息
wecom-pro bot show work

# 删除指定 Bot
wecom-pro bot remove personal
```

### 品类调用

每个品类作为独立子命令使用。不传方法名时列出该品类下所有可用工具，传方法名时调用指定工具。

#### 单 Bot 模式

```bash
# 列出品类下的所有工具
wecom-pro <category>

# 调用品类下的指定工具
wecom-pro <category> <method> [json_args]
```

示例：

```bash
# 列出通讯录品类下的工具
wecom-pro contact

# 列出待办品类下的工具
wecom-pro todo

# 调用工具（传 JSON 参数）
wecom-pro contact get_userlist '{}'

# 调用工具（无参数）
wecom-pro contact get_userlist
```

#### 多 Bot 模式

所有品类命令都支持 `--bot` / `-b` 参数，用于指定使用的 Bot。

```bash
# 使用指定 Bot 调用工具
wecom-pro <category> --bot <bot_id> <method> [json_args]

# 列出指定 Bot 可用的工具
wecom-pro msg --bot work
```

示例：

```bash
# 使用 work Bot 发送消息
wecom-pro msg --bot work send_message '{"chat_type": 1, "chatid": "zhangsan", "msgtype": "text", "text": {"content": "工作通知"}}'

# 使用 personal Bot 创建待办
wecom-pro todo --bot personal create_todo '{"content": "个人事项", "todo_status": 1}'

# 使用 work Bot 查询通讯录
wecom-pro contact --bot work get_userlist '{}'

# 使用 personal Bot 管理文档
wecom-pro doc --bot personal create_doc '{"doc_type": 3, "doc_name": "个人笔记"}'
```

## 品类与能力一览

### contact — 通讯录

| 工具 | 说明 |
|------|------|
| `get_userlist` | 获取当前用户可见范围内的通讯录成员（userid、姓名、别名） |

```bash
# 获取全量通讯录成员
wecom-pro contact get_userlist '{}'

# 使用指定 Bot 获取通讯录
wecom-pro contact --bot work get_userlist '{}'
```

### todo — 待办

| 工具 | 说明 |
|------|------|
| `get_todo_list` | 查询待办列表，支持按时间过滤和分页 |
| `get_todo_detail` | 根据待办 ID 批量查询完整详情 |
| `create_todo` | 创建待办，可指定内容、分派人、提醒时间 |
| `update_todo` | 更新待办内容、状态、分派人或提醒时间 |
| `delete_todo` | 删除待办（不可撤销） |
| `change_todo_user_status` | 变更当前用户在待办中的状态 |

```bash
# 查询待办列表
wecom-pro todo get_todo_list '{}'

# 使用 work Bot 创建待办
wecom-pro todo --bot work create_todo '{"content": "完成Q2规划文档", "remind_time": "2026-06-01 09:00:00"}'

# 使用 personal Bot 创建待办
wecom-pro todo --bot personal create_todo '{"content": "个人学习计划", "todo_status": 1}'

# 批量查询待办详情
wecom-pro todo get_todo_detail '{"todo_id_list": ["TODO_ID_1", "TODO_ID_2"]}'

# 标记待办完成
wecom-pro todo update_todo '{"todo_id": "TODO_ID", "todo_status": 0}'

# 删除待办
wecom-pro todo delete_todo '{"todo_id": "TODO_ID"}'
```

### meeting — 会议

| 工具 | 说明 |
|------|------|
| `create_meeting` | 创建预约会议，支持设置参数、邀请参与人、安全设置 |
| `cancel_meeting` | 取消指定的预约会议 |
| `set_invite_meeting_members` | 更新会议受邀成员（全量覆盖） |
| `list_user_meetings` | 查询用户在时间范围内的会议列表（当日前后 30 天） |
| `get_meeting_info` | 获取会议完整详情 |

```bash
# 查询本周会议
wecom-pro meeting list_user_meetings '{"begin_datetime": "2026-03-23 00:00", "end_datetime": "2026-03-29 23:59", "limit": 100}'

# 使用 work Bot 创建会议
wecom-pro meeting --bot work create_meeting '{"title": "技术方案评审", "meeting_start_datetime": "2026-03-30 15:00", "meeting_duration": 3600, "invitees": {"userid": ["zhangsan", "lisi"]}}'

# 使用 personal Bot 创建会议
wecom-pro meeting --bot personal create_meeting '{"title": "个人学习讨论", "meeting_start_datetime": "2026-03-30 20:00", "meeting_duration": 1800, "invitees": {"userid": ["zhangsan"]}}'

# 获取会议详情
wecom-pro meeting get_meeting_info '{"meetingid": "MEETING_ID"}'

# 取消会议
wecom-pro meeting cancel_meeting '{"meetingid": "MEETING_ID"}'
```

### msg — 消息

| 工具 | 说明 |
|------|------|
| `get_msg_chat_list` | 按时间范围查询有消息的会话列表 |
| `get_message` | 拉取会话消息记录（支持文本/图片/文件/语音/视频） |
| `get_msg_media` | 下载消息中的多媒体文件到本地 |
| `send_message` | 向单聊或群聊发送文本消息 |

```bash
# 获取最近一周会话列表
wecom-pro msg get_msg_chat_list '{"begin_time": "2026-03-22 00:00:00", "end_time": "2026-03-29 23:59:59"}'

# 使用 work Bot 发送工作消息
wecom-pro msg --bot work send_message '{"chat_type": 1, "chatid": "zhangsan", "msgtype": "text", "text": {"content": "项目进度更新"}}'

# 使用 personal Bot 发送个人消息
wecom-pro msg --bot personal send_message '{"chat_type": 1, "chatid": "lisi", "msgtype": "text", "text": {"content": "周末聚会安排"}}'

# 拉取聊天记录
wecom-pro msg get_message '{"chat_type": 1, "chatid": "zhangsan", "begin_time": "2026-03-29 09:00:00", "end_time": "2026-03-29 18:00:00"}'

# 下载多媒体文件
wecom-pro msg get_msg_media '{"media_id": "MEDIA_ID"}'
```

### schedule — 日程

| 工具 | 说明 |
|------|------|
| `get_schedule_list_by_range` | 查询时间范围内的日程 ID 列表（当日前后 30 天） |
| `get_schedule_detail` | 批量获取日程详情（1~50 个） |
| `create_schedule` | 创建日程，支持设置提醒、参与人 |
| `update_schedule` | 修改日程（只传需修改的字段） |
| `cancel_schedule` | 取消日程 |
| `add_schedule_attendees` | 添加日程参与人 |
| `del_schedule_attendees` | 移除日程参与人 |
| `check_availability` | 查询多成员闲忙状态（1~10 人） |

```bash
# 查询今天的日程
wecom-pro schedule get_schedule_list_by_range '{"start_time": "2026-03-29 00:00:00", "end_time": "2026-03-29 23:59:59"}'

# 使用 work Bot 创建工作日程
wecom-pro schedule --bot work create_schedule '{"schedule": {"start_time": "2026-03-30 14:00:00", "end_time": "2026-03-30 15:00:00", "summary": "需求评审", "attendees": [{"userid": "zhangsan"}], "reminders": {"is_remind": 1, "remind_before_event_secs": 900, "timezone": 8}}}'

# 使用 personal Bot 创建个人日程
wecom-pro schedule --bot personal create_schedule '{"schedule": {"start_time": "2026-04-05 19:00:00", "end_time": "2026-04-05 20:00:00", "summary": "健身", "reminders": {"is_remind": 1, "remind_before_event_secs": 1800, "timezone": 8}}}'

# 查询闲忙
wecom-pro schedule check_availability '{"check_user_list": ["zhangsan", "lisi"], "start_time": "2026-03-30 09:00:00", "end_time": "2026-03-30 18:00:00"}'
```

### doc — 文档

| 工具 | 说明 |
|------|------|
| `create_doc` | 创建文档（doc_type=3） |
| `get_doc_content` | 获取文档内容（Markdown 格式，异步轮询） |
| `edit_doc_content` | 用 Markdown 覆写文档正文 |

```bash
# 创建文档
wecom-pro doc create_doc '{"doc_type": 3, "doc_name": "项目周报"}'

# 使用 work Bot 创建工作文档
wecom-pro doc --bot work create_doc '{"doc_type": 3, "doc_name": "工作总结"}'

# 使用 personal Bot 创建个人文档
wecom-pro doc --bot personal create_doc '{"doc_type": 3, "doc_name": "个人笔记"}'

# 读取文档内容（首次调用）
wecom-pro doc get_doc_content '{"docid": "DOC_ID", "type": 2}'

# 读取文档内容（轮询，携带 task_id）
wecom-pro doc get_doc_content '{"docid": "DOC_ID", "type": 2, "task_id": "TASK_ID"}'

# 使用 work Bot 编辑文档内容
wecom-pro doc --bot work edit_doc_content '{"docid": "DOC_ID", "content": "# 项目周报\n\n## 本周进展\n...", "content_type": 1}'
```

### doc — 智能表格

| 工具 | 说明 |
|------|------|
| `create_doc` | 创建智能表格（通过 doc create_doc，doc_type=10） |
| `smartsheet_get_sheet` | 查询智能表格的所有子表 |
| `smartsheet_add_sheet` | 添加子表 |
| `smartsheet_update_sheet` | 修改子表标题 |
| `smartsheet_delete_sheet` | 删除子表（不可逆） |
| `smartsheet_get_fields` | 查询子表的字段/列信息 |
| `smartsheet_add_fields` | 添加字段/列 |
| `smartsheet_update_fields` | 更新字段标题 |
| `smartsheet_delete_fields` | 删除字段/列（不可逆） |
| `smartsheet_get_records` | 查询子表全部记录 |
| `smartsheet_add_records` | 添加记录 |
| `smartsheet_update_records` | 更新记录 |
| `smartsheet_delete_records` | 删除记录（不可逆） |

```bash
# 创建智能表格
wecom-pro doc create_doc '{"doc_type": 10, "doc_name": "任务跟踪表"}'

# 查询智能表格子表
wecom-pro doc smartsheet_get_sheet '{"docid": "DOC_ID"}'

# 查询子表字段信息
wecom-pro doc smartsheet_get_fields '{"docid": "DOC_ID", "sheet_id": "SHEET_ID"}'

# 添加子表字段
wecom-pro doc smartsheet_add_fields '{"docid": "DOC_ID", "sheet_id": "SHEET_ID", "fields": [{"field_title": "状态", "field_type": "FIELD_TYPE_SINGLE_SELECT"}]}'

# 查询子表记录
wecom-pro doc smartsheet_get_records '{"docid": "DOC_ID", "sheet_id": "SHEET_ID"}'

# 添加记录
wecom-pro doc smartsheet_add_records '{"docid": "DOC_ID", "sheet_id": "SHEET_ID", "records": [{"values": {"标题": [{"type": "text", "text": "新任务"}]}}]}'

# 更新记录
wecom-pro doc smartsheet_update_records '{"docid": "DOC_ID", "sheet_id": "SHEET_ID", "key_type":"CELL_VALUE_KEY_TYPE_FIELD_TITLE", "records": [{"record_id": "RECORD_ID", "values": {"标题": [{"type": "text", "text": "已更新"}]}}]}'

# 删除记录
wecom-pro doc smartsheet_delete_records '{"docid": "DOC_ID", "sheet_id": "SHEET_ID", "record_ids": ["RECORD_ID"]}'
```

## 多 Bot 功能

### 概述

wecom-pro 支持配置多个企业微信机器人，实现工作与个人账号分离，不同业务场景使用不同 Bot。

### 使用场景

- **工作/个人分离**: 使用 `work` bot 处理工作任务，`personal` bot 处理个人事务
- **多项目管理**: 为不同项目配置专用 bot，实现数据和权限隔离
- **团队协作**: 不同团队使用各自的 bot，避免操作冲突
- **环境隔离**: 开发、测试、生产环境使用不同的 bot

### 配置管理

#### 初始化多个 Bot

```bash
# 初始化默认 Bot（单 Bot 模式）
wecom-pro init

# 初始化多个 Bot
wecom-pro init --bot work
wecom-pro init --bot personal
wecom-pro init --bot project-a
```

#### 管理 Bot 配置

```bash
# 列出所有已配置的 Bot
wecom-pro bot list

# 显示 Bot 详细信息
wecom-pro bot show work

# 删除指定 Bot
wecom-pro bot remove project-a
```

### 使用方式

#### 基本语法

所有业务命令都支持 `--bot` / `-b` 参数：

```bash
wecom-pro <category> --bot <bot_id> <method> [json_args]
```

#### 实际示例

```bash
# 工作场景 - 使用 work bot
wecom-pro msg --bot work send_message '{"chat_type": 1, "chatid": "zhangsan", "msgtype": "text", "text": {"content": "会议开始提醒"}}'
wecom-pro todo --bot work create_todo '{"content": "提交月度报告", "todo_status": 1, "remind_time": "2026-04-30 17:00:00"}'
wecom-pro meeting --bot work create_meeting '{"title": "周会", "meeting_start_datetime": "2026-04-07 14:00", "meeting_duration": 3600, "invitees": {"userid": ["team"]}}'

# 个人场景 - 使用 personal bot
wecom-pro msg --bot personal send_message '{"chat_type": 1, "chatid": "lisi", "msgtype": "text", "text": {"content": "周末聚餐"}}'
wecom-pro todo --bot personal create_todo '{"content": "买菜", "todo_status": 1, "remind_time": "2026-04-05 18:00:00"}'
wecom-pro schedule --bot personal create_schedule '{"schedule": {"start_time": "2026-04-06 14:00:00", "end_time": "2026-04-06 16:00:00", "summary": "朋友聚会"}}'

# 项目场景 - 使用项目专用 bot
wecom-pro doc --bot project-a create_doc '{"doc_type": 3, "doc_name": "项目A需求文档"}'
wecom-pro doc --bot project-a edit_doc_content '{"docid": "DOC_ID", "content": "# 项目A需求\\n\\n## 功能列表\\n1. ...", "content_type": 1}'
```

### 配置存储

多 Bot 配置独立存储在：

```
~/.config/wecom/
├── bot.enc              # default bot 的配置
├── bots/                # 其他 bot 的配置目录
│   ├── work.enc
│   ├── personal.enc
│   └── project-a.enc
└── mcp_configs/         # MCP 配置缓存
    ├── default.enc
    ├── work.enc
    ├── personal.enc
    └── project-a.enc
```

### 向后兼容性

多 Bot 功能完全向后兼容：

- ✅ 不指定 `--bot` 参数时，默认使用 `default` bot
- ✅ 所有现有命令和脚本继续正常工作
- ✅ 渐进式采用，可按需启用多 Bot 功能

### 技术特性

- **配置隔离**: 每个 bot 独立的配置文件和 MCP 缓存
- **加密存储**: AES-256-GCM 加密，Keyring 优先
- **原子操作**: 文件写入保证原子性，防止数据损坏
- **权限安全**: 敏感文件权限设为 0o600
- **跨平台**: 支持 macOS/Linux/Windows

## 项目说明

### 与原项目的关系

本项目（wecom-pro）是从 [WecomTeam/wecom-cli](https://github.com/WecomTeam/wecom-cli) fork 而来，主要增加了以下功能：

- **多 Bot 支持**: 可以配置多个企业微信机器人，实现工作与个人账号分离
- **配置隔离**: 每个 bot 独立的配置文件和 MCP 缓存
- **更好的灵活性**: 支持不同业务场景使用不同的 bot

### 技术栈

- **语言**: Rust
- **版本**: v0.1.3+
- **作者**: Liangdi <wu@liangdi.me>
- **仓库**: [https://github.com/Liangdi/wecom-pro](https://github.com/Liangdi/wecom-pro)

### 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

本项目基于 **MIT 许可证** 开源。
