# biolab-cli 项目更新方案

## 背景与目标

`biolab-cli` 当前已经具备基础命令能力：登录、用户信息、订单、模板、库存、课题组管理，以及初步的 Agent skill 安装命令。项目下一步目标不是简单堆命令，而是把 CLI 整理成“人和 AI Agent 都能稳定使用”的产品型工具。

本方案用于约束后续更新：先明确模块边界和验收标准，再按阶段实施。

## 当前状态

已具备：

- Rust CLI 主入口和 clap 子命令路由。
- Feishu OAuth 登录和本地 token 存储。
- 订单、模板、库存、课题组、用户相关 API 调用。
- JSON 文件创建订单和模板。
- 初步 Agent skill：`biolab skills install/check/path`。

主要问题：

- `client.rs` 同时承担 HTTP transport、API endpoint、响应解包和文件上传，模块职责过重。
- 响应解析会在数组解析失败时退化为空数组，容易隐藏后端契约变化。
- 输出层没有统一 envelope，`OutputFormat` 在部分函数里没有实际生效。
- Agent skill 缺少 `references/` 领域说明文档，`CLAUDE.md` 提到的引用文件尚未落地。
- 模板和订单工作流没有打通，仍要求用户/Agent 自己拼 JSON。
- 认证流程偏人工阻塞，不适合 Agent split-flow。

## 模块边界规划

### 1. API 响应模块

目标文件：`src/api_response.rs`

职责：

- 统一处理 HTTP 响应状态码。
- 统一处理后端 `{ data: ... }` envelope。
- 区分空结果和解析失败。
- 保留可读错误，便于 Agent 判断下一步。

验收标准：

- `client.rs` 不再直接定义 `parse_response`、`extract_array`、`extract_object`。
- 数组解析失败必须返回错误，不能静默返回 `[]`。
- 现有类型反序列化测试通过。

### 2. 服务层模块

目标目录：`src/services/`

建议拆分：

- `services/users.rs`
- `services/orders.rs`
- `services/templates.rs`
- `services/inventory.rs`
- `services/lab.rs`

职责：

- 每个服务模块只负责对应业务域 API。
- `client.rs` 保留底层 HTTP 能力和通用上传/下载。
- commands 调用服务能力，而不是依赖巨型 client。

验收标准：

- 单个文件不再承载全部业务 API。
- 新增业务命令时只需要改对应服务模块和命令模块。

### 3. 输出契约模块

目标文件：`src/output.rs`

职责：

- 明确 text/json 两种输出。
- JSON 输出可被 Agent 稳定解析。
- 错误输出尽量结构化，避免把错误当普通文本散落。

验收标准：

- `print_result` 尊重 `OutputFormat` 或改名为明确的 JSON 输出函数。
- 下载类命令、创建类命令、列表类命令的输出策略一致。

### 4. Agent Skill 文档模块

目标目录：`skills/biolab-api/references/`

建议文件：

- `orders.md`
- `inventory.md`
- `templates.md`
- `lab.md`
- `users.md`

职责：

- 为 Agent 提供领域规则、常用命令、危险操作、JSON 输入示例。
- `SKILL.md` 做路由和总规则，细节放 references。

验收标准：

- `CLAUDE.md` 中提到的 references 文件实际存在。
- `SKILL.md` 明确要求在复杂订单/库存/课题组操作前读取对应 reference。

### 5. 业务工作流命令

建议新增：

- `orders validate <file>`
- `orders create-primer --from-template <template-id> --items <file>`
- `orders create-sequencing --from-template <template-id> --items <file>`
- `orders preview <file>`
- `templates export <id> <file>`

职责：

- 降低 Agent 手写完整 JSON 的失败率。
- 把模板能力真正接入订单创建流程。

验收标准：

- 常见下单流程可以通过模板加 items 文件完成。
- JSON 输入错误能在本地提前发现。

## 本轮执行范围

本轮先做低风险、能立即改善结构的更新：

1. 新增 `src/api_response.rs`。
2. 将 `client.rs` 中的响应解析函数迁移到 `api_response.rs`。
3. 修正数组解析失败静默吞错的问题。
4. 新增 `skills/biolab-api/references/` 基础文档。
5. 更新 `SKILL.md`，让 Agent 知道何时读取 reference。

暂不做：

- 大规模服务层拆分。
- 认证 split-flow。
- 输出 envelope 重构。
- 新增订单模板工作流命令。

## 验收标准

本轮完成后应满足：

- `cargo fmt --check` 通过。
- `cargo test --offline` 通过。
- `client.rs` 响应解包职责被移出。
- Agent skill references 存在并与 `CLAUDE.md` 对齐。
- 不引入新的依赖。

## 风险与注意事项

- 当前 `Cargo.lock` 在本地解析时可能出现依赖解析漂移，非本轮功能变更不应提交 lockfile 噪音。
- 现有中文文案在部分终端输出中会显示乱码，需要后续单独做编码与文案清理。
- 后端 API envelope 的真实形态如果不止 `{ data }`，后续应补充更多 fixture 测试。
