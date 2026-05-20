# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with this repository.

## Project Overview

**biolab-cli** — A Rust CLI client for the Biolab lab management system (primer synthesis + sequencing orders, inventory, lab administration). Replaces the Python scripts embedded in `.claude/skills/biolab-api/scripts/`.

The system communicates with a FastAPI backend at `http://8.136.56.203/api/v1` using Feishu OAuth for authentication.

## Common Commands

```bash
# Build
cargo build --release

# Run
./target/release/biolab <cmd>    # release binary

# Help
biolab --help
biolab orders --help
biolab inventory --help

# Install AI agent skills for this project
biolab skills install
biolab skills check -f json

# Login (Feishu OAuth)
biolab login
biolab status
biolab logout

# Output in JSON (for machine parsing)
biolab me -f json
biolab orders list -f json
```

## Architecture

### Source Layout

```
Cargo.toml          # Rust project, binary: biolab, lib: biolab
src/
├── main.rs         # CLI entry — clap subcommand router
├── lib.rs          # Public API re-exports
├── config.rs       # Token management (env → file → OAuth), base URL
├── client.rs       # HTTP client (reqwest), all API methods
├── types.rs        # Serde request/response structs
├── auth.rs         # Feishu OAuth login flow (tiny_http callback server)
├── output.rs       # Formatting: JSON (--format json) vs colored text
└── commands/
    ├── users.rs    # me / update-me / change-password
    ├── orders.rs   # list / get / create-primer / create-sequencing / update / resend / download / template upload
    ├── templates.rs# CRUD + default for order-info templates
    ├── inventory.rs# list / get / stats / checkin / checkout / locations
    └── lab.rs      # lab info / members / invite / join / approval rules
```

### Key Patterns

- **Credential chain**: `BIOLAB_TOKEN` env var → `~/.biolab_token` file → interactive OAuth
- **Token storage**: `~/.biolab_token`, valid 8 days
- **HTTP client**: `BiolabClient` wraps reqwest with Bearer token injection, handles JSON responses with fallback to `data` envelope
- **Output modes**: `-f json` for machine-readable, default text for human (colored status badges)
- **Commands pattern**: Each `commands/*.rs` module defines clap subcommands + `run()` async function, called from `main.rs` dispatcher
- **Agent skills**: `biolab skills install` copies the bundled `skills/biolab-api/SKILL.md` into `.claude/skills/biolab-api` and `.codex/skills/biolab-api`, then writes a version stamp for `biolab skills check`

### API Base URL

Default: `http://8.136.56.203/api/v1` — overrideable via `BIOLAB_BASE_URL` env var.

## Business Domain

### Order Status Machine
```
pending → ordered → received → stored
(待下单)  (已下单)  (已收货)   (已入库)
```

### Order Types
| Type | Supplier(s) |
|------|-------------|
| `primer_synthesis` | `sangon` (生工) / `biosune` (铂尚) |
| `sequencing` | `biosune` |

### Lab Permission Model
Five workflow roles: `pi` > `procurement` > `finance` > `warehouse` > `member`

### Reference Docs
Detailed API schemas are bundled in `skills/biolab-api/references/` and installed into `.claude/skills/biolab-api/references/`:
- `orders.md` — Order schemas, status machine, supplier differences
- `inventory.md` — Stock/checkin/checkout schemas
- `templates.md` — Template fields for order defaults
- `lab.md` — Lab CRUD, member management, approval rules
- `users.md` — User info, permission model, signup

## CI

`.github/workflows/release.yml` builds for 5 targets on push:
- Linux (x86_64 + arm64 via musl)
- Windows (x86_64)
- macOS (x86_64 + arm64)

Tagged pushes (e.g. `v0.1.0`) auto-create GitHub Releases with binaries.
