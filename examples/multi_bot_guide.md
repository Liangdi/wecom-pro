# 多 Bot 功能使用指南

本指南介绍如何使用 wecom-cli 的多 Bot 功能，实现工作与个人账号分离，以及不同业务场景的 Bot 隔离。

## 目录

- [快速开始](#快速开始)
- [使用场景](#使用场景)
- [完整示例](#完整示例)
- [最佳实践](#最佳实践)
- [故障排除](#故障排除)

## 快速开始

### 1. 初始化多个 Bot

```bash
# 初始化工作 Bot
wecom-cli init --bot work

# 初始化个人 Bot  
wecom-cli init --bot personal
```

### 2. 查看已配置的 Bot

```bash
wecom-cli bot list
```

输出示例：
```
已配置的 Bot：
  - work (创建时间: 大约 2 小时前) [默认]
  - personal (创建时间: 大约 1 小时前)
```

### 3. 使用指定 Bot

```bash
# 使用 work Bot 发送工作消息
wecom-cli msg --bot work send_message '{
  "chat_type": 1,
  "chatid": "zhangsan",
  "msgtype": "text",
  "text": {"content": "项目会议已开始"}
}'
```

## 使用场景

### 场景一：工作与个人分离

**需求**: 工作时间处理工作任务，个人时间处理个人事务，避免混淆。

```bash
# 工作时间 - 创建工作待办
wecom-cli todo --bot work create_todo '{
  "content": "完成Q2技术方案",
  "todo_status": 1,
  "remind_time": "2026-04-15 09:00:00"
}'

# 个人时间 - 创建个人待办
wecom-cli todo --bot personal create_todo '{
  "content": "周末家庭聚餐",
  "todo_status": 1,
  "remind_time": "2026-04-08 18:00:00"
}'
```

### 场景二：多项目管理

**需求**: 不同项目使用各自的 Bot，实现数据隔离和权限管理。

```bash
# 项目A - 创建项目文档
wecom-cli doc --bot project-a create_doc '{
  "doc_type": 3,
  "doc_name": "项目A需求文档"
}'

# 项目B - 创建项目文档
wecom-cli doc --bot project-b create_doc '{
  "doc_type": 3,
  "doc_name": "项目B技术方案"
}'

# 项目A - 发送团队消息
wecom-cli msg --bot project-a send_message '{
  "chat_type": 2,
  "chatid": "project_a_group",
  "msgtype": "text",
  "text": {"content": "项目A需求评审会议明天下午2点"}
}'
```

### 场景三：团队协作

**需求**: 不同团队使用各自的 Bot，避免操作冲突和权限问题。

```bash
# 技术团队 - 创建技术会议
wecom-cli meeting --bot tech-team create_meeting '{
  "title": "技术方案评审",
  "meeting_start_datetime": "2026-04-10 14:00:00",
  "meeting_duration": 3600,
  "invitees": {"userid": ["dev1", "dev2", "dev3"]}
}'

# 产品团队 - 创建产品会议
wecom-cli meeting --bot product-team create_meeting '{
  "title": "产品需求讨论",
  "meeting_start_datetime": "2026-04-10 16:00:00",
  "meeting_duration": 3600,
  "invitees": {"userid": ["pm1", "pm2"]}
}'
```

### 场景四：环境隔离

**需求**: 开发、测试、生产环境使用不同的 Bot，避免测试数据污染生产环境。

```bash
# 开发环境 - 创建测试数据
wecom-cli doc --bot dev create_doc '{
  "doc_type": 10,
  "doc_name": "测试数据表"
}'

# 生产环境 - 创建正式文档
wecom-cli doc --bot prod create_doc '{
  "doc_type": 3,
  "doc_name": "用户手册"
}'
```

## 完整示例

### 示例一：日常工作流

```bash
# 1. 查看工作待办
wecom-cli todo --bot work get_todo_list '{}'

# 2. 发送工作通知
wecom-cli msg --bot work send_message '{
  "chat_type": 2,
  "chatid": "team_group",
  "msgtype": "text",
  "text": {"content": "今日站会已更新，请查看"}
}'

# 3. 创建工作会议
wecom-cli meeting --bot work create_meeting '{
  "title": "周会",
  "meeting_start_datetime": "2026-04-07 14:00:00",
  "meeting_duration": 3600,
  "invitees": {"userid": ["team"]},
  "description": "讨论本周工作进展"
}'

# 4. 更新工作文档
wecom-cli doc --bot work edit_doc_content '{
  "docid": "WORK_DOC_ID",
  "content": "# 工作周报\\n\\n## 本周进展\\n...",
  "content_type": 1
}'
```

### 示例二：个人事务管理

```bash
# 1. 创建个人待办
wecom-cli todo --bot personal create_todo '{
  "content": "周末家庭聚餐",
  "todo_status": 1,
  "remind_time": "2026-04-08 18:00:00"
}'

# 2. 发送个人消息
wecom-cli msg --bot personal send_message '{
  "chat_type": 1,
  "chatid": "friend",
  "msgtype": "text",
  "text": {"content": "这周末有空吗？想约你吃饭"}
}'

# 3. 创建个人日程
wecom-cli schedule --bot personal create_schedule '{
  "schedule": {
    "start_time": "2026-04-08 18:00:00",
    "end_time": "2026-04-08 20:00:00",
    "summary": "家庭聚餐",
    "reminders": {
      "is_remind": 1,
      "remind_before_event_secs": 3600,
      "timezone": 8
    }
  }
}'
```

### 示例三：项目管理

```bash
# 项目初始化
wecom-cli init --bot project-alpha

# 创建项目文档
wecom-cli doc --bot project-alpha create_doc '{
  "doc_type": 3,
  "doc_name": "Project Alpha 需求文档"
}'

# 创建项目待办
wecom-cli todo --bot project-alpha create_todo '{
  "content": "完成需求分析",
  "todo_status": 1,
  "remind_time": "2026-04-15 09:00:00"
}'

# 创建项目会议
wecom-cli meeting --bot project-alpha create_meeting '{
  "title": "Project Alpha 启动会",
  "meeting_start_datetime": "2026-04-12 10:00:00",
  "meeting_duration": 7200,
  "invitees": {"userid": ["stakeholder1", "stakeholder2"]}
}'

# 查看项目配置
wecom-cli bot show project-alpha
```

## 最佳实践

### 1. Bot 命名规范

推荐使用清晰、描述性的 Bot ID：

- `work` - 工作账号
- `personal` - 个人账号
- `project-{name}` - 项目专用
- `{team}-team` - 团队专用
- `{env}-{type}` - 环境隔离 (如 `dev-db`, `prod-app`)

### 2. 权限管理

- 为不同 Bot 配置不同的权限和功能范围
- 工作场景 Bot 用于业务操作
- 个人 Bot 用于私人事务
- 测试 Bot 仅在开发环境使用

### 3. 数据隔离

- 不同 Bot 的待办、会议、文档相互独立
- 避免在错误的 Bot 下操作数据
- 定期清理不再使用的 Bot 配置

### 4. 脚本和自动化

在脚本中明确指定 Bot：

```bash
#!/bin/bash
# 工作日报脚本
BOT_ID="work"

# 发送日报消息
wecom-cli msg --bot "$BOT_ID" send_message '{
  "chat_type": 2,
  "chatid": "team_group",
  "msgtype": "text",
  "text": {"content": "今日工作进展：..."}
}'

# 查看团队待办
wecom-cli todo --bot "$BOT_ID" get_todo_list '{}'
```

### 5. 错误处理

```bash
# 检查 Bot 是否存在
if ! wecom-cli bot show "$BOT_ID" &> /dev/null; then
  echo "Error: Bot '$BOT_ID' not found"
  echo "Please run: wecom-cli init --bot $BOT_ID"
  exit 1
fi

# 使用指定 Bot 执行操作
wecom-cli msg --bot "$BOT_ID" send_message '...'
```

## 故障排除

### 问题 1: Bot 不存在

**错误信息**: `Bot 'xxx' 不存在，请先添加`

**解决方案**:
```bash
# 初始化指定的 Bot
wecom-cli init --bot xxx
```

### 问题 2: MCP 配置缺失

**错误信息**: `未找到 Bot 'xxx' 的 MCP 配置缓存`

**解决方案**:
```bash
# 重新初始化 Bot，会自动获取 MCP 配置
wecom-cli init --bot xxx
```

### 问题 3: 权限不足

**错误信息**: API 返回权限错误

**解决方案**:
- 检查 Bot 的权限配置
- 确认 Bot 有访问对应资源的权限
- 联系管理员配置 Bot 权限

### 问题 4: 配置损坏

**错误信息**: 解密失败、配置读取错误

**解决方案**:
```bash
# 删除损坏的 Bot 配置
wecom-cli bot remove xxx

# 重新初始化
wecom-cli init --bot xxx
```

## 高级用法

### 批量操作

```bash
# 对多个 Bot 执行相同操作
for bot in work personal; do
  wecom-cli msg --bot "$bot" send_message '{
    "chat_type": 1,
    "chatid": "admin",
    "msgtype": "text",
    "text": {"content": "系统维护通知"}
  }'
done
```

### 定时任务

```bash
# crontab 定时任务示例
# 每天早上9点检查工作待办
0 9 * * * /usr/local/bin/wecom-cli todo --bot work get_todo_list '{}' > /tmp/work_todos.log

# 每周一早上创建周会
0 9 * * 1 /usr/local/bin/wecom-cli meeting --bot work create_meeting '{
  "title": "周会",
  "meeting_start_datetime": "...",
  "meeting_duration": 3600
}'
```

### 环境变量

```bash
# 为不同会话设置不同的默认 Bot
export WECOM_DEFAULT_BOT="work"
wecom-cli msg send_message '{}'  # 使用 work bot

export WECOM_DEFAULT_BOT="personal"
wecom-cli todo get_todo_list '{}'  # 使用 personal bot
```

## 总结

多 Bot 功能提供了灵活的账号隔离和管理能力，适用于各种复杂的业务场景。通过合理配置和使用，可以：

- ✅ 实现工作与个人事务分离
- ✅ 支持多项目、多团队协作
- ✅ 提供环境隔离和权限管理
- ✅ 提高操作安全性和数据隔离

开始使用多 Bot 功能，让企业微信 CLI 更好地服务于你的多样化需求！
