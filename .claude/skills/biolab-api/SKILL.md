---
name: biolab-api
description: Interact with the biolab order system (primer synthesis + sequencing), inventory, and user settings. Use whenever the user mentions orders, primers, 引物, sequencing, 测序, inventory, 库存, reagent stocks, lab orders, sample management, order info templates, 下单, 订单, supplier (sangon/biosune), or anything related to managing a molecular biology lab's purchasing workflow — even if they don't explicitly say "biolab" or "API."
compatibility: biolab-cli (Rust binary), Feishu account required for authentication
---

# Biolab 实验管理系统

Base URL: `http://8.136.56.203/api/v1`

## 参考文档导航

每个业务领域有独立的参考文档，按需查阅——不要在每次对话中全部加载。

| 用户想做的事 | 参考文档 |
|-------------|---------|
| 创建/查看订单、了解字段含义、状态流转 | [references/orders.md](references/orders.md) |
| 管理信息模板（默认值、地址、联系方式） | [references/templates.md](references/templates.md) |
| 库存查询、出入库、存储位置 | [references/inventory.md](references/inventory.md) |
| 课题组管理、成员邀请、审批规则 | [references/lab.md](references/lab.md) |
| 用户设置、权限模型 | [references/users.md](references/users.md) |

> 下单时必须查阅 orders.md 了解字段 schema；涉及模板操作时查阅 templates.md。

---

## CLI 工具

所有命令通过 `biolab` CLI 执行。如果二进制不在 PATH 中，使用完整路径：

```bash
./target/release/biolab <command>
```

### 全局选项

- `-f json` — 输出 JSON 格式（脚本解析用）
- `-f text` — 人类可读的格式化输出（默认）
- `-h` / `--help` — 显示帮助

---

## 1. 登录

本系统仅支持飞书 OAuth 认证。Token 有效期 8 天，保存到 `~/.biolab_token`，后续无需重复登录。

```bash
# 飞书 OAuth 登录（打开浏览器完成认证）
biolab login

# 检查登录状态
biolab status

# 登出（删除本地 token）
biolab logout
```

### 远程会话（VS Code Remote / WSL / SSH）

`biolab login` 会启动一个本地 HTTP 服务器等待飞书回调。用户的浏览器可能无法访问 Agent 侧的 localhost。

**症状**：login 一直等待，2 分钟后超时退出。

**解决方法**：让用户在**自己的本地终端**中运行 `biolab login`。如果上述方法不可行，备选方案：从回调 URL 中提取 token 参数，手动写入 `~/.biolab_token`。

---

## 2. 常用命令速查

### 账户

```bash
# 查看当前用户信息（姓名、手机、邮箱、权限）
biolab me

# 更新个人信息
biolab me update '{"phone_number":"15186909747"}'

# 修改密码
biolab me change-password --current '旧密码' --new '新密码'
```

### 订单

```bash
# 订单列表（查看所有订单状态）
biolab orders list

# 订单详情（含引物/样品列表）
biolab orders get <ID>

# 创建引物合成订单（从 JSON 文件）
biolab orders create-primer <json文件>

# 创建测序订单（从 JSON 文件）
biolab orders create-sequencing <json文件>

# 修改订单状态（如标记已收货）
biolab orders update <ID> '{"status":"received"}'

# 重发邮件（pending 状态的订单一键重发）
biolab orders resend <ID>

# 下载订单 Excel（归档/报销用）
biolab orders download <ID> [输出路径]

# 下载供应商 Excel 模板
biolab orders download-primer-template [路径]
biolab orders download-sequencing-template [路径]

# 上传 Excel 解析（引物/测序）
biolab orders upload-primer-excel <文件路径>
biolab orders upload-sequencing-excel <文件路径>
```

### 信息模板

```bash
# 列出我的所有模板
biolab templates list

# 查看默认模板（下单前必查，获取单位/地址/PI等默认值）
biolab templates get-default primer_synthesis

# 模板详情
biolab templates get <ID>

# 从 JSON 文件创建模板
biolab templates create <json文件>

# 更新模板
biolab templates update <ID> <json文件>

# 删除模板
biolab templates delete <ID>

# 设为默认模板
biolab templates set-default <ID>
```

### 库存

```bash
# 库存列表
biolab inventory list

# 按名称筛选
biolab inventory list --primer-name 'xxx'

# 仅显示低库存
biolab inventory list --low-stock

# 库存详情（含交易记录）
biolab inventory get <ID>

# 库存统计（总数、低库存数）
biolab inventory stats

# 入库
biolab inventory checkin <ID> --quantity 5 --purpose "实验用"

# 出库
biolab inventory checkout <ID> --quantity 2 --purpose "PCR" --experiment-ref "EXP-001"

# 存储位置列表
biolab inventory locations

# 创建存储位置
biolab inventory create-location <名称> [--parent-id <父位置ID>]
```

### 课题组

```bash
# 课题组信息
biolab lab info

# 创建课题组
biolab lab create <名称>

# 更新课题组设置
biolab lab update '{"require_approval":true}'

# 成员列表
biolab lab members

# 修改成员角色
biolab lab update-role <user_id> procurement

# 移除成员
biolab lab remove-member <user_id>

# 邀请成员
biolab lab invite <邮箱> [member]

# 查看邀请
biolab lab invitations

# 接受/拒绝邀请
biolab lab accept-invite <invitation_id>
biolab lab decline-invite <invitation_id>

# 申请加入课题组
biolab lab join <lab_id> [member]

# 查看入组申请（PI）
biolab lab applications

# 批准/拒绝申请（PI）
biolab lab approve-app <application_id>
biolab lab reject-app <application_id>

# 审批规则列表
biolab lab approval-rules

# 添加/删除审批规则
biolab lab add-rule '{"order_type":"primer_synthesis","max_price":500,"approver_role":"finance"}'
biolab lab remove-rule <rule_id>
```

---

## 3. 下单

### Agent 下单流程

下单前按顺序检查以下步骤。每一步都有其目的——跳过步骤会导致订单缺少必要信息被后端拒绝。

1. **检查登录** — `biolab status`。未登录就下单 = 401 失败。
2. **获取用户信息** — `biolab me -f json`。检查 `phone_number` 是否为空——不少账户注册时未填手机号，模板也未设置的话订单缺少联系人信息会被拒绝。如果为空，先 `biolab me update '{"phone_number":"用户手机号"}'`。
3. **检查默认模板** — `biolab templates get-default primer_synthesis -f json`。模板存储了单位、地址、PI、付款方式等固定信息。如果返回 404 或缺少关键字段（phone、地址、单位），系统缺少这些信息无法生成完整订单——先让用户补充模板再继续。
4. **确认选项** — 供应商 / 纯化方式 / 规格等业务选项不能假设默认值，不同实验室有不同的偏好和供应商合同。
5. **提取引物** — 从用户提供的文档（Excel/文本/聊天记录）中提取引物列表。
6. **构建 JSON 并提交** — 合并模板默认值 + 用户确认的选项 + 提取的引物，写入临时 JSON 文件后通过 `biolab orders create-primer <文件>` 提交。

> 模板字段详见 [references/templates.md](references/templates.md)，订单 Schema 详见 [references/orders.md](references/orders.md)。

### 交互式下单（用户直接在终端使用）

编写临时 JSON 文件后用 `biolab orders create-primer` 提交。典型订单 JSON 结构：

```json
{
  "type": "primer_synthesis",
  "supplier_name": "sangon",
  "customer_name": "张三",
  "customer_phone": "13800000000",
  "customer_email": "zhang@example.com",
  "company_name": "复旦大学",
  "invoice_title": "复旦大学",
  "principal_investigator": "张教授",
  "payment_method": "月结",
  "recipient_address": "上海市杨浦区邯郸路220号",
  "weekend_delivery": true,
  "partial_delivery": true,
  "confidential": true,
  "items": [
    {
      "primer_name": "FWD-GAPDH",
      "sequence": "ATGGAGAAGGCTGGGGCTCATT",
      "base_count": 22,
      "purification_method": "HPLC",
      "nmoles": 25
    }
  ]
}
```

> 供应商可用值：`sangon`（生工）、`biosune`（铂尚）。纯化方式：`HAP`/`PAGE`/`HPLC`/`ULTRAPAGE`。

### 测序订单

```json
{
  "type": "sequencing",
  "supplier_name": "biosune",
  "customer_name": "张三",
  "customer_phone": "13800000000",
  "customer_email": "zhang@example.com",
  "items": [
    {
      "name": "sample-1",
      "type": "质粒",
      "seq_vector": "pUC19",
      "universal": true
    }
  ]
}
```

> 可选附带引物合成：在订单中添加 `"primer_items": [...]` 数组。

---

## 4. 信息模板

模板存储下单时的固定信息（单位、地址、PI、付款方式），让用户不必每次重复填写。**Agent 下单前必须拉取默认模板**——没模板 = 订单缺关键字段。

常见工作流：
1. 首次下单：无模板 → 收集用户的单位/地址/PI/手机 → `biolab templates create <json文件>` 创建并设为默认
2. 后续下单：`biolab templates get-default primer_synthesis -f json` 拉取 → 确认后直接填入订单 JSON

模板字段详见 [references/templates.md](references/templates.md)。

---

## 5. 库存

引物收货后可入库（checkin），使用时出库（checkout）。库存的典型生命周期：

```
订单标记 received → 逐条 checkin 入库 → 实验中 checkout 出库 → 库存不足时提醒补货
```

查询低库存用 `biolab inventory list --low-stock`——帮助判断是否需要重新订购。

Schema 详见 [references/inventory.md](references/inventory.md)。

---

## 6. 课题组（Lab）

系统支持课题组管理模式：每个用户属于一个课题组，有五种工作流角色（PI/采购/仓管/财务/成员），支持分级审批规则。

### 快速检查

Agent 在处理与课题组相关的请求时，先通过 `biolab me -f json` 查看用户的 `lab_id`、`role`、`lab` 字段：

- `lab_id` 为 null → 用户尚未加入课题组，需引导创建或接受邀请
- `role` 为 "pi" → 课题组负责人，可管理成员、配置审批规则
- `role` 为 "member" 且 lab 开启审批 → 创建的订单需要审批

完整接口和 schema 详见 [references/lab.md](references/lab.md)。

---

## 7. 用户设置

用户信息中的 `phone_number` 对下单至关重要——Agent 应在下单前检查并提示补全。

权限模型和完整接口详见 [references/users.md](references/users.md)。
