---
name: scitex-users
version: 0.1.0
description: "Use when checking Scientex login status, reading or updating the authenticated user's profile, changing password, logging out, or preparing user contact fields for orders."
metadata:
  requires:
    bins: ["scitex"]
  cliHelp: "scitex me --help"
---

# Scientex Users

**Before starting, read `../scitex-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

## Commands

```bash
scitex status
scitex me -f json
scitex me update '{"phone_number":"13800000000"}' -f json
scitex me change-password --current <OLD_PASSWORD> --new <NEW_PASSWORD>
scitex logout
```

## Schema

Inspect `<SCIENTEX_BASE_URL>/openapi.json` before updating user fields.

Relevant schemas:

- `UserUpdateMe`: optional `full_name`, `email`, `phone_number`
- `UpdatePassword`: requires `current_password`, `new_password`

## Rules

- Do not print tokens, passwords, or secrets.
- Prefer `scitex status` before assuming token validity.
- Confirm before updating profile fields.
- Never ask the user to paste a password into chat unless the user explicitly chooses that flow.
- Use `scitex me -f json` to prefill order contact fields only after user confirmation.
- Do not invent admin user-management commands from backend `users` endpoints; this skill is scoped to the authenticated account unless the CLI exposes more commands.
