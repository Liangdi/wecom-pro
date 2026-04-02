---
name: wecomcli-manage-smartsheet-data
description: 企业微信智能表格数据（记录）管理技能。提供智能表格记录的增删改查能力。适用场景：(1) 查询子表全部记录 (2) 添加一行或多行记录 (3) 更新已有记录 (4) 删除记录。当用户需要读取表格数据、写入新数据、修改或删除表格行时触发此 Skill。支持通过 docid 或文档 URL 定位文档。
metadata:
  requires:
    bins: ["wecom-pro"]
  cliHelp: "wecom-pro doc --help"
---

# 企业微信智能表格数据管理

> `wecom-pro` 是企业微信提供的命令行程序，所有操作通过执行 `wecom-pro` 命令完成。

管理智能表格中的记录（行数据）。所有接口支持通过 `docid` 或 `url` 二选一定位文档。

## CLI 调用方式

通过 `wecom-pro` 调用，品类为 `doc`：

```bash
wecom-pro doc <tool_name> '<json_params>'
```

## 返回格式说明

所有接口返回 JSON 对象，包含以下公共字段：

| 字段 | 类型 | 说明 |
|------|------|------|
| `errcode` | integer | 返回码，`0` 表示成功，非 `0` 表示失败 |
| `errmsg` | string | 错误信息，成功时为 `"ok"` |

当 `errcode` 不为 `0` 时，说明接口调用失败，可重试 1 次；若仍失败，将 `errcode` 和 `errmsg` 展示给用户。

### smartsheet_get_records

查询子表全部记录。

- 通过 sheetid：
```bash
wecom-pro doc smartsheet_get_records '{"docid": "DOCID", "sheet_id": "SHEETID"}'
```
- 或通过 URL：
```bash
wecom-pro doc smartsheet_get_records '{"url": "https://doc.weixin.qq.com/smartsheet/xxx", "sheet_id": "SHEETID"}'
```

参见 [API 详情](references/api-get-records.md)。

### smartsheet_add_records

添加一行或多行记录，单次建议 500 行内。

**调用前**必须先了解目标表的字段类型（通过 `smartsheet_get_fields`），重点关注 `field_type`。对于单选/多选（Option）字段，需注意匹配已有选项的 `id`。

```bash
wecom-pro doc smartsheet_add_records '{"docid": "DOCID", "sheet_id": "SHEETID", "records": [{"values": {"任务名称": [{"type": "text", "text": "完成需求文档"}], "优先级": [{"text": "高"}]}}]}'
```

各字段类型的值格式参见 [单元格值格式参考](references/cell-value-formats.md)。

### smartsheet_update_records

更新一行或多行记录，单次建议在 500 行内。需提供 record_id（通过 `smartsheet_get_records` 获取）。支持通过 `key_type` 指定 values 的 key 使用字段标题或字段 ID：

- `CELL_VALUE_KEY_TYPE_FIELD_TITLE`：key 为字段标题
- `CELL_VALUE_KEY_TYPE_FIELD_ID`：key 为字段 ID

```bash
wecom-pro doc smartsheet_update_records '{"docid": "DOCID", "sheet_id": "SHEETID", "key_type": "CELL_VALUE_KEY_TYPE_FIELD_TITLE", "records": [{"record_id": "RECORDID", "values": {"任务名称": [{"type": "text", "text": "更新后的内容"}]}}]}'
```

**注意**：创建时间、最后编辑时间、创建人、最后编辑人字段不可更新。

### smartsheet_delete_records

删除一行或多行记录，单次必须在 500 行内。**操作不可逆**。record_id 通过 `smartsheet_get_records` 获取。

```bash
wecom-pro doc smartsheet_delete_records '{"docid": "DOCID", "sheet_id": "SHEETID", "record_ids": ["RECORDID1", "RECORDID2"]}'
```

## 典型工作流

1. **读取数据** → 
```bash
wecom-pro doc smartsheet_get_records '{"docid":"DOCID","sheet_id":"SHEETID"}'
```
2. **写入数据** → 先 `smartsheet_get_fields` 了解列类型 → 若涉及成员（USER）字段，先通过 `wecomcli-lookup-contact` 的 `get_userlist` 查找人员 userid → `smartsheet_add_records` 写入
3. **更新数据** → 先 `smartsheet_get_records` 获取 record_id → 若涉及成员（USER）字段，先通过 `wecomcli-lookup-contact` 的 `get_userlist` 查找人员 userid → `smartsheet_update_records` 更新
4. **删除数据** → 先 `smartsheet_get_records` 确认 record_id → `smartsheet_delete_records` 删除

> **注意**：成员（USER）类型字段需要填写 `user_id`，不能直接使用姓名。必须先通过 `wecomcli-lookup-contact` 技能的 `get_userlist` 接口按姓名查找到对应的 `userid` 后再使用。
