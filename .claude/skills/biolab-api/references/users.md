# Users 参考

所有用户接口 base: `/users`

## 权限模型

### 课题组角色

系统使用五种工作流角色（`user.role`）：

| 角色 | 标识 | 权限 |
|------|------|------|
| PI | `pi` | 管理成员、审批订单、全部权限 |
| 采购 | `procurement` | 创建/发送订单、查看价格 |
| 仓管 | `warehouse` | 库存出入库、管理存放位置 |
| 财务 | `finance` | 查看价格、审批匹配规则的订单 |
| 成员 | `member` | 创建申请单、查看库存 |

### 超级管理员

- `is_superuser=true` 的用户拥有全部权限，可管理所有用户
- 超管可以：列出/创建/编辑/删除任何用户
- 超管在课题组相关操作中也拥有最高权限

### 普通用户（无课题组）

- `lab_id` 为 null 的用户只能操作自己的资源（me 系列接口）
- 这类用户需要创建课题组或接受邀请后才能体验完整功能

## 接口一览

| 方法 | 路径 | 权限 | 说明 |
|------|------|------|------|
| GET | `/users/` | 超管 | 用户列表 |
| POST | `/users/` | 超管 | 创建用户 |
| GET | `/users/me` | 登录用户 | 当前用户信息（含 lab_id、role、lab） |
| PATCH | `/users/me` | 登录用户 | 更新自己的信息 |
| PATCH | `/users/me/password` | 登录用户 | 修改密码 |
| DELETE | `/users/me` | 登录用户 | 注销账号 |
| GET | `/users/{id}` | 超管或自己 | 查看用户 |
| PATCH | `/users/{id}` | 超管 | 更新用户 |
| DELETE | `/users/{id}` | 超管 | 删除用户（会级联删其 orders） |
| POST | `/users/signup` | 公开 | 注册（首个用户自动创建课题组并成为 PI） |

## UserCreate / UserUpdate

```json
{
  "email": "user@example.com",
  "password": "xxx",
  "full_name": "姓名",
  "phone_number": "13800000000",
  "is_active": true,
  "is_superuser": false,
  "lab_id": null,
  "role": null
}
```

> `lab_id` 和 `role` 通常由课题组管理接口设置，不建议在用户创建/更新时直接修改。

## UserPublic (response)

```json
{
  "id": "uuid",
  "email": "user@example.com",
  "is_active": true,
  "is_superuser": false,
  "full_name": "姓名",
  "phone_number": "13800000000",
  "feishu_open_id": null,
  "local_role": null,
  "lab_id": "uuid 或 null",
  "role": "member",
  "lab": {
    "id": "uuid",
    "name": "张课题组",
    "pi_id": "uuid",
    "require_approval": false,
    "created_at": "2026-05-19T...",
    "member_count": 5
  },
  "created_at": "2026-05-11T..."
}
```

## 课题组相关操作

课题组管理接口详见 [references/lab.md](references/lab.md)，主要包括：

- 创建/查看/更新课题组
- 成员管理（列表、改角色、移除）
- 邀请流程（PI 邀请 → 用户接受/拒绝）
- 申请流程（用户申请 → PI 批准/拒绝）
- 审批规则配置