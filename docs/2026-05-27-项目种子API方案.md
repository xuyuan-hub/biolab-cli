# Project Slug Seed/Tashan API CLI Plan

Date: 2026-05-27

This plan covers the newly exposed project-scoped APIs under `/api/v1/project/{slug}/...`. These are separate from the existing project-management APIs under `/api/v1/projects...`.

## Summary

Add a singular project command namespace:

```bash
biolab project <SLUG> <COMMAND>
```

Example:

```bash
biolab project tashan germplasm list -f json
biolab project tashan planting list -f json
```

Keep the existing plural namespace unchanged:

```bash
biolab projects ...
```

`biolab projects` manages project records and members by project id. `biolab project <slug>` operates inside one project by slug, such as the Tashan seed/germplasm workflows.

## Confirmed Backend Endpoints

The following paths were confirmed from OpenAPI:

| Domain | Method | Backend path | Proposed CLI |
| --- | --- | --- | --- |
| Project lookup | GET | `/projects/by-slug/{slug}` | `biolab project <SLUG> info` |
| Germplasm | GET | `/project/{slug}/germplasm` | `biolab project <SLUG> germplasm list` |
| Germplasm | POST | `/project/{slug}/germplasm` | `biolab project <SLUG> germplasm create '<JSON>'` |
| Germplasm | GET | `/project/{slug}/germplasm/{germplasm_id}` | `biolab project <SLUG> germplasm get <ID>` |
| Germplasm | PATCH | `/project/{slug}/germplasm/{germplasm_id}` | `biolab project <SLUG> germplasm update <ID> '<JSON>'` |
| Germplasm | DELETE | `/project/{slug}/germplasm/{germplasm_id}` | `biolab project <SLUG> germplasm delete <ID>` |
| Germplasm files | GET | `/project/{slug}/germplasm/{germplasm_id}/sequencing-files` | `biolab project <SLUG> germplasm sequencing-files <ID>` |
| Germplasm stocks | GET | `/project/{slug}/germplasm/{germplasm_id}/stocks` | `biolab project <SLUG> germplasm stocks <ID>` |
| Planting | GET | `/project/{slug}/planting` | `biolab project <SLUG> planting list` |
| Planting | POST | `/project/{slug}/planting` | `biolab project <SLUG> planting create '<JSON>'` |
| Planting | GET | `/project/{slug}/planting/{order_id}` | `biolab project <SLUG> planting get <ID>` |
| Planting | PATCH | `/project/{slug}/planting/{order_id}` | `biolab project <SLUG> planting update <ID> '<JSON>'` |
| Planting harvests | GET | `/project/{slug}/planting/{order_id}/harvests` | `biolab project <SLUG> planting harvests <ID>` |
| Planting harvests | POST | `/project/{slug}/planting/{order_id}/harvests` | `biolab project <SLUG> planting create-harvest <ID> '<JSON>'` |
| Planting items | GET | `/project/{slug}/planting/{order_id}/items` | `biolab project <SLUG> planting items <ID>` |

## CLI Design

Top-level shape:

```bash
biolab project <SLUG> info
biolab project <SLUG> germplasm <SUBCOMMAND>
biolab project <SLUG> planting <SUBCOMMAND>
```

Germplasm commands:

```bash
biolab project <SLUG> germplasm list [--skip N] [--limit N] [--search TEXT] [--filters JSON]
biolab project <SLUG> germplasm get <GERMPLASM_ID>
biolab project <SLUG> germplasm create '<JSON>'
biolab project <SLUG> germplasm update <GERMPLASM_ID> '<JSON>'
biolab project <SLUG> germplasm delete <GERMPLASM_ID>
biolab project <SLUG> germplasm sequencing-files <GERMPLASM_ID>
biolab project <SLUG> germplasm stocks <GERMPLASM_ID>
```

Planting commands:

```bash
biolab project <SLUG> planting list [--skip N] [--limit N]
biolab project <SLUG> planting get <ORDER_ID>
biolab project <SLUG> planting create '<JSON>'
biolab project <SLUG> planting update <ORDER_ID> '<JSON>'
biolab project <SLUG> planting items <ORDER_ID>
biolab project <SLUG> planting harvests <ORDER_ID>
biolab project <SLUG> planting create-harvest <ORDER_ID> '<JSON>'
```

Defaults:

- Germplasm list defaults: `skip=0`, `limit=10`, matching OpenAPI.
- Planting list defaults: use OpenAPI defaults, expected `skip=0`; confirm `limit` from schema before implementation.
- `--filters` is passed as a raw query string containing JSON array text, matching backend description: `[{field, operator, value, combine}]`.
- JSON create/update payloads are passed through as `serde_json::Value`.
- Responses should initially be `serde_json::Value` or `Vec<serde_json::Value>` to keep CLI resilient while schemas evolve.

## Implementation Plan

Add code in a new singular project module:

- `src/commands/project.rs`
  - Add `ProjectArgs { slug, command }`.
  - Add nested subcommands `Germplasm` and `Planting`.
  - Parse JSON payload strings for create/update/harvest creation.

- `src/services/project.rs`
  - Add path builders for `/project/{slug}/germplasm...` and `/project/{slug}/planting...`.
  - Add `get_project_by_slug` for `/projects/by-slug/{slug}`.
  - Add service methods for every confirmed endpoint.
  - Use existing `url_encode` helper for `search` and `filters` query values.

- `src/commands/mod.rs`, `src/services/mod.rs`, `src/main.rs`
  - Register singular `project` module.
  - Add top-level `Project(project::ProjectArgs)` command.

- `README.md`
  - Add command examples under a new "Project-scoped workflows" section.
  - Keep `biolab projects ...` documented separately as project administration.

## Safety And Exclusions

- Do not add users admin, signup, password recovery, or audit/log commands.
- Do not replace existing `biolab projects ...`; add singular `biolab project <SLUG> ...`.
- Destructive commands are limited to germplasm delete and project member removal already present in other command groups; CLI implementation should not add extra confirmation prompts because current CLI style delegates confirmation to the caller/agent.
- Do not invent field-level typed request structs for germplasm/planting until schemas stabilize.

## Validation Plan

Run local checks:

```powershell
cargo fmt
cargo test
cargo build
```

Inspect help:

```powershell
target\debug\biolab.exe project --help
target\debug\biolab.exe project tashan --help
target\debug\biolab.exe project tashan germplasm --help
target\debug\biolab.exe project tashan planting --help
```

Live read-only smoke tests, after login:

```powershell
biolab project tashan info -f json
biolab project tashan germplasm list -f json
biolab project tashan planting list -f json
```

Mutating tests require explicit test data:

```powershell
biolab project tashan germplasm create '<JSON>' -f json
biolab project tashan germplasm update <GERMPLASM_ID> '<JSON>' -f json
biolab project tashan germplasm delete <GERMPLASM_ID> -f json
biolab project tashan planting create '<JSON>' -f json
biolab project tashan planting update <ORDER_ID> '<JSON>' -f json
biolab project tashan planting create-harvest <ORDER_ID> '<JSON>' -f json
```

Acceptance criteria:

- `biolab project <SLUG> ...` commands appear in help.
- `biolab projects ...` remains unchanged.
- `users`, `signup`, password recovery, and audit/log commands remain unavailable.
- Unit tests cover path builders and query-string construction for `search` and `filters`.

---

## TODO List

全部 15 个 endpoint 已实现（commits 8d1662c / 22fec67 / 02c1f0f）：

### 代码文件完成情况
- [x] `src/services/project.rs` — 所有路径构建器和服务方法（含 3 个单元测试，commit 8d1662c）
- [x] `src/commands/project.rs` — ProjectArgs / GermplasmCommand / PlantingCommand 完整实现（含 8 个命令解析单元测试，commit 22fec67）
- [x] `src/lib.rs` / `src/main.rs` — 模块注册，`biolab project` 顶级命令接入
- [x] `skills/biolab-tashan/SKILL.md` — Agent Skill 文档（commit 02c1f0f）

### 命令列表（全部完成）
- [x] `biolab project <SLUG> info` — `get_project_by_slug`
- [x] `biolab project <SLUG> germplasm list` — `list_project_germplasm`
- [x] `biolab project <SLUG> germplasm get <ID>` — `get_project_germplasm`
- [x] `biolab project <SLUG> germplasm create` — `create_project_germplasm`
- [x] `biolab project <SLUG> germplasm update <ID>` — `update_project_germplasm`
- [x] `biolab project <SLUG> germplasm delete <ID>` — `delete_project_germplasm`
- [x] `biolab project <SLUG> germplasm sequencing-files <ID>` — `list_project_germplasm_sequencing_files`
- [x] `biolab project <SLUG> germplasm stocks <ID>` — `list_project_germplasm_stocks`
- [x] `biolab project <SLUG> planting list` — `list_project_planting_orders`
- [x] `biolab project <SLUG> planting get <ID>` — `get_project_planting_order`
- [x] `biolab project <SLUG> planting create` — `create_project_planting_order`
- [x] `biolab project <SLUG> planting update <ID>` — `update_project_planting_order`
- [x] `biolab project <SLUG> planting items <ID>` — `list_project_planting_items`
- [x] `biolab project <SLUG> planting harvests <ID>` — `list_project_planting_harvests`
- [x] `biolab project <SLUG> planting create-harvest <ID>` — `create_project_planting_harvest`
