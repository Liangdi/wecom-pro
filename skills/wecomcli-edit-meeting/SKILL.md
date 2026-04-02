---
name: wecomcli-edit-meeting
description: 企业微信会议管理技能, 支持取消会议和更新会议受邀成员. 当用户需要"取消会议", "删除会议", "修改会议成员", "添加会议参与人", "移除会议成员"时触发.
metadata:
  requires:
    bins: ["wecom-pro"]
  cliHelp: "wecom-pro meeting --help"
---
# 企业微信会议管理技能

> `wecom-pro` 是企业微信提供的命令行程序，所有操作通过执行 `wecom-pro` 命令完成。


## 概述

wecomcli-edit-meeting 提供企业微信会议管理能力, 包含以下功能:

1. **取消会议** - 取消指定的预约会议
2. **更新会议受邀成员** - 修改会议的参与人列表

## 命令调用方式

调用指定命令：
```bash
wecom-pro meeting <tool_name> '<json_params>'
```

---

## 命令详细说明

### 1. 取消会议 (cancel_meeting)

取消指定的预约会议.

#### 调用命令

```bash
wecom-pro meeting cancel_meeting '{"meetingid": "<会议id>"}'
```

#### 入参说明

| 参数              | 类型   | 必填 | 说明                               |
| ----------------- | ------ | ---- | ---------------------------------- |
| `meetingid`     | string | 是   | 会议 ID, 通过 `wecomcli-get-meeting` 技能获取 |

#### 返回参数

```json
{
  "errcode": 0,
  "errmsg": "ok"
}
```

---

### 2. 更新会议受邀成员 (set_invite_meeting_members)

更新会议的受邀成员列表 (全量覆盖).

#### 调用命令

```bash
wecom-pro meeting set_invite_meeting_members '{"meetingid": "<会议id>", "invitees": [{"userid": "lisi"}, {"userid": "wangwu"}]}'
```

#### 入参说明

| 参数          | 类型   | 必填 | 说明                                   |
| ------------- | ------ | ---- | -------------------------------------- |
| `meetingid` | string | 是   | 会议 ID, 通过 `wecomcli-get-meeting` 技能获取                          |
| `invitees`  | array  | 否   | 受邀成员列表, 每项包含 `userid` 字段 |

> **注意**: invitees 为全量覆盖, 传入的列表将替换现有成员列表.
> invitees 的 userid 通过 `wecomcli-lookup-contact` 技能获取

#### 返回参数

```json
{
  "errcode": 0,
  "errmsg": "ok"
}
```

---

## 典型工作流

### 工作流 1: 取消会议

**示例**: 用户说 "帮我取消明天的技术方案评审会议"

**步骤:**

1. **定位会议**: 通过 `wecomcli-get-meeting` 技能查询会议列表 + 关键词匹配找到目标会议.
2. **直接执行取消**:

```bash
wecom-pro meeting cancel_meeting '{"meetingid": "<target_meetingid>"}'
```

3. **展示结果**:

```
✅ 会议已取消: 技术方案评审
```

---

### 工作流 2: 更新会议成员

**示例**: 用户说 "把王五加到技术方案评审会议里"

**步骤:**

1. **定位会议**: 通过 `wecomcli-get-meeting` 技能查询会议列表 + 匹配找到目标会议.
2. **获取当前受邀成员**: `set_invite_meeting_members` 为全量覆盖, 必须先通过 `wecomcli-get-meeting` 技能的 `get_meeting_info` 获取会议详情, 获取现有成员后再合并.
3. **通讯录查询**: 调用 `wecomcli-lookup-contact` 技能获取通讯录成员, 按姓名筛选出王五的 userid.

```bash
wecom-pro contact get_userlist '{}'
```

在返回的 `userlist` 中筛选 `name` 包含 "王五" 的成员, 获取其 `userid`.

4. **合并成员列表**: 将现有成员 + 新增成员合并 (全量覆盖).
5. **执行更新**:

```bash
wecom-pro meeting set_invite_meeting_members '{"meetingid": "<target_meetingid>", "invitees": [{"userid": "zhangsan"}, {"userid": "lisi"}, {"userid": "wangwu"}]}'
```

6. **展示结果**:

```
✅ 会议成员已更新: 技术方案评审
👥 当前成员: 张三, 李四, 王五
```

---

## 注意事项

- **参与人仅支持企业内成员**, 不支持外部人员
- **通讯录查询**: 涉及参与人时, 需先通过 `wecomcli-lookup-contact` 技能的 `get_userlist` 接口获取全量通讯录成员, 再按姓名/别名本地筛选匹配出对应的 `userid`. 该接口无入参, 返回当前用户可见范围内的成员列表 (含 `userid`, `name`, `alias`)
- **定位会议**: 管理操作需先通过 `wecomcli-get-meeting` 技能查询到目标会议的 meetingid
- **成员更新为全量覆盖**: `set_invite_meeting_members` 传入的列表将替换现有成员列表, 需先获取当前成员再合并
