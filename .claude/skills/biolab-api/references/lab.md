# Lab 参考

所有课题组接口 base: `/lab`

## 权限模型

系统有五种工作流角色：

| 角色 | 标识 | 权限 |
|------|------|------|
| PI | `pi` | 管理成员、审批订单、全部权限 |
| 采购 | `procurement` | 创建/发送订单、查看价格 |
| 仓管 | `warehouse` | 库存出入库、管理存放位置 |
| 财务 | `finance` | 查看价格、审批订单（规则匹配时） |
| 成员 | `member` | 创建申请单、查看库存 |

PI 和 superuser 始终拥有全部权限。

## 接口一览

### 课题组 CRUD

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/lab` | 获取当前用户所属课题组 |
| POST | `/lab/create` | 创建课题组（自动成为 PI），需未加入任何课题组 |
| PATCH | `/lab` | PI 更新课题组（name, require_approval） |

### 成员管理

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/lab/members` | 列出成员及角色 |
| PATCH | `/lab/members/{user_id}` | PI 修改成员角色 |
| DELETE | `/lab/members/{user_id}` | PI 移除成员 |

### 邀请流程（PI → 用户）

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/lab/invite` | PI 发送邀请（创建 pending 状态邀请） |
| GET | `/lab/invitations` | 查看邀请（PI 看已发送，用户看收到的） |
| POST | `/lab/invitations/{id}/accept` | 接受邀请并加入课题组 |
| POST | `/lab/invitations/{id}/decline` | 拒绝邀请 |

### 申请流程（用户 → PI）

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/lab/join/{lab_id}` | 用户申请加入课题组（创建 applied 状态） |
| GET | `/lab/applications` | PI 查看待处理入组申请 |
| POST | `/lab/applications/{id}/approve` | PI 批准申请 |
| POST | `/lab/applications/{id}/reject` | PI 拒绝申请 |

### 审批规则

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/lab/approval-rules` | 列出课题组审批规则 |
| POST | `/lab/approval-rules` | PI 添加审批规则 |
| DELETE | `/lab/approval-rules/{id}` | PI 删除审批规则 |

## Schema

### LabPublic (response)

```json
{
  "id": "uuid",
  "name": "张课题组",
  "pi_id": "uuid",
  "require_approval": false,
  "created_at": "2026-05-19T...",
  "member_count": 5
}
```

### LabCreate (request)

```json
{
  "name": "课题组名称"
}
```

### LabUpdate (request)

```json
{
  "name": "新名称",
  "require_approval": true
}
```

### LabMemberPublic (response)

```json
{
  "id": "uuid",
  "email": "user@example.com",
  "full_name": "张三",
  "role": "member",
  "is_active": true
}
```

### LabMemberUpdate (request)

```json
{
  "role": "procurement"
}
```

### LabInviteRequest (request)

```json
{
  "email": "user@example.com",
  "role": "member"
}
```

### LabInvitationPublic (response)

```json
{
  "id": "uuid",
  "lab_id": "uuid",
  "lab_name": "张课题组",
  "inviter_name": "张教授",
  "invitee_email": "student@example.com",
  "role": "member",
  "status": "pending",
  "created_at": "2026-05-19T..."
}
```

status 取值：`pending`（待确认）、`accepted`（已接受）、`declined`（已拒绝）、`applied`（已申请）。

### LabJoinRequest (request)

```json
{
  "role": "member"
}
```

### LabApprovalRuleCreate (request)

```json
{
  "order_type": "primer_synthesis",
  "max_price": 500.0,
  "approver_role": "finance"
}
```

- `order_type`：`"primer_synthesis"` / `"sequencing"` / `null`（全部类型）
- `max_price`：价格上限，`null` 表示不限金额
- `approver_role`：`"pi"` / `"finance"` / `"procurement"`

规则按 sort_order 顺序匹配，命中第一条即生效。未匹配到任何规则时默认 PI 审批。

### LabApprovalRulePublic (response)

```json
{
  "id": "uuid",
  "lab_id": "uuid",
  "order_type": "primer_synthesis",
  "max_price": 500.0,
  "approver_role": "finance",
  "sort_order": 1
}
```

## 审批规则示例

PI 配置以下规则后：

```
#1 引物合成 ≤¥500 → 财务审批
#2 引物合成 ≤¥2000 → 采购审批  
#3 测序不限 → PI 审批
#4 全部不限 → PI 审批（兜底）
```

效果：
- 成员提交 ¥300 引物订单 → 财务可直接审批
- 成员提交 ¥800 引物订单 → 采购可直接审批
- 成员提交 ¥3000 引物订单 → PI 审批（#2 价格不匹配，#4 兜底）
- 成员提交测序订单（任何金额）→ PI 审批（#3 匹配）