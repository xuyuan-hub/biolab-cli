# OpenAPI CLI Command Coverage Plan

Date: 2026-05-25

This plan tracks CLI coverage for backend endpoints that exist in the current OpenAPI spec but are not fully exposed by `biolab-cli`.

## Scope

Add CLI commands for the newly confirmed backend capabilities in these domains:

- Orders: stats, pending approvals, approve, reject, send
- Lab: lab-wide orders, lab order stats, lab shared inventory
- Inventory: stock transactions, inventory preferences
- Projects: project CRUD and project membership management

Do not implement Users admin APIs, signup, or password recovery in this round. The backend exposes user administration and signup/recovery endpoints, but those interfaces must not be opened in the CLI yet.

Do not add audit or audit-log related commands in this round. If the OpenAPI spec exposes audit trails, operation logs, admin audit records, or similar endpoints, keep them out of scope until explicitly requested.

## Confirmed Backend Endpoints

The following paths were confirmed from `GET /api/v1/openapi.json`:

| Domain | Method | Backend path | Proposed command |
| --- | --- | --- | --- |
| Orders | GET | `/orders/stats` | `biolab orders stats` |
| Orders | GET | `/orders/approvals/pending` | `biolab orders pending-approvals` |
| Orders | POST | `/orders/{order_id}/approve` | `biolab orders approve <ID>` |
| Orders | POST | `/orders/{order_id}/reject` | `biolab orders reject <ID>` |
| Orders | POST | `/orders/{order_id}/send` | `biolab orders send <ID>` |
| Lab | GET | `/lab/orders` | `biolab lab orders` |
| Lab | GET | `/lab/orders/stats` | `biolab lab orders-stats` |
| Lab | GET | `/lab/inventory/stocks` | `biolab lab inventory` |
| Inventory | GET | `/inventory/stocks/{stock_id}/transactions` | `biolab inventory transactions <ID>` |
| Inventory | GET | `/inventory/preferences` | `biolab inventory preferences` |
| Inventory | PUT | `/inventory/preferences` | `biolab inventory set-preferences '<JSON>'` |
| Projects | GET | `/projects` | `biolab projects list` |
| Projects | POST | `/projects` | `biolab projects create '<JSON>'` |
| Projects | GET | `/projects/{project_id}` | `biolab projects get <ID>` |
| Projects | PATCH | `/projects/{project_id}` | `biolab projects update <ID> '<JSON>'` |
| Projects | GET | `/projects/{project_id}/members` | `biolab projects members <ID>` |
| Projects | POST | `/projects/{project_id}/members` | `biolab projects add-member <PROJECT_ID> <USER_ID> [role]` |
| Projects | DELETE | `/projects/{project_id}/members/{user_id}` | `biolab projects remove-member <PROJECT_ID> <USER_ID>` |

## Priority

Phase 1 should cover the high-frequency operational workflow:

1. `biolab orders stats`
2. `biolab orders pending-approvals`
3. `biolab orders approve <ID>`
4. `biolab orders reject <ID>`
5. `biolab inventory transactions <ID>`

Phase 2 should cover lab-wide views:

1. `biolab lab orders`
2. `biolab lab orders-stats`
3. `biolab lab inventory`

Phase 3 should cover inventory settings:

1. `biolab inventory preferences`
2. `biolab inventory set-preferences '<JSON>'`

Phase 4 should cover project management:

1. `biolab projects list`
2. `biolab projects get <ID>`
3. `biolab projects create '<JSON>'`
4. `biolab projects update <ID> '<JSON>'`
5. `biolab projects members <ID>`
6. `biolab projects add-member <PROJECT_ID> <USER_ID> [role]`
7. `biolab projects remove-member <PROJECT_ID> <USER_ID>`

## Implementation Notes

- Prefer returning `serde_json::Value` for stats, preferences, approval action responses, and other schemas that may evolve.
- Skip audit/log endpoints even if they appear in the OpenAPI spec.
- Reuse existing typed models where they already fit:
  - `Order` for order list/detail style responses.
  - `Stock` for inventory stock lists.
  - `Transaction` for inventory transaction lists.
- Keep existing `biolab orders resend <ID>` as a compatibility alias for `/orders/{order_id}/send`.
- Add `biolab orders send <ID>` as the clearer OpenAPI-aligned command.
- Use JSON objects for project create/update so the CLI can pass through backend fields such as `name`, `description`, `budget`, dates, `pi_id`, `application_file_url`, and `status`.
- Project endpoints expose optional `X-Current-Lab`; the CLI does not add a lab-selection flag in this round and relies on backend defaults/current context.
- Keep Users admin APIs, signup, and password recovery out of scope until explicitly requested.

## File-Level Plan

Expected code touch points:

- `src/services/orders.rs`
  - Add `get_order_stats`.
  - Add `list_pending_approvals`.
  - Add `send_order`.
  - Add `approve_order`.
  - Add `reject_order`.

- `src/commands/orders.rs`
  - Add `Stats`.
  - Add `PendingApprovals`.
  - Add `Send`.
  - Add `Approve`.
  - Add `Reject`.

- `src/services/inventory.rs`
  - Add `list_stock_transactions`.
  - Add `get_inventory_preferences`.
  - Add `set_inventory_preferences`.

- `src/commands/inventory.rs`
  - Add `Transactions`.
  - Add `Preferences`.
  - Add `SetPreferences`.

- `src/services/lab.rs`
  - Add `list_lab_orders`.
  - Add `get_lab_order_stats`.
  - Add `list_lab_inventory`.

- `src/commands/lab.rs`
  - Add `Orders`.
  - Add `OrdersStats`.
  - Add `Inventory`.

- `src/services/projects.rs`
  - Add project CRUD path helpers.
  - Add member list/add/remove path helpers.

- `src/commands/projects.rs` and `src/main.rs`
  - Add top-level `Projects` command group.
  - Add list/get/create/update/members/add-member/remove-member commands.

- `README.md` and skill docs
  - Document new commands after implementation is verified.

## Validation Plan

Run these checks locally:

```powershell
cargo fmt
cargo test
cargo build
```

Then inspect command help:

```powershell
biolab orders --help
biolab inventory --help
biolab lab --help
biolab projects --help
biolab me --help
```

For live API validation, use JSON output where possible:

```powershell
biolab orders stats -f json
biolab orders pending-approvals -f json
biolab inventory transactions <STOCK_ID> -f json
biolab inventory preferences -f json
biolab projects list -f json
biolab projects members <PROJECT_ID> -f json
```

Mutating commands should be tested only with explicit confirmation and suitable test data:

```powershell
biolab orders approve <ORDER_ID> -f json
biolab orders reject <ORDER_ID> -f json
biolab inventory set-preferences '{"low_stock_threshold":5}' -f json
biolab projects create '{"name":"Grant 2026"}' -f json
biolab projects update <PROJECT_ID> '{"status":"active"}' -f json
biolab projects add-member <PROJECT_ID> <USER_ID> member -f json
biolab projects remove-member <PROJECT_ID> <USER_ID> -f json
```

## Acceptance Criteria

- Every command in scope appears in `--help`.
- Read commands return structured JSON with `-f json`.
- Text mode remains usable for order and inventory list style responses.
- Existing commands keep working, especially `orders resend`, `orders list`, `inventory list`, and `lab members`.
- Unit tests cover newly added path builders and request body helpers.
- Users admin APIs, signup, and password recovery remain undocumented and unimplemented in the CLI.
- Audit/log related endpoints remain undocumented and unimplemented in the CLI.
