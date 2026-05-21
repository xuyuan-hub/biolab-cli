---
name: biolab-users
version: 0.1.0
description: "Use when checking Biolab login status, reading or updating the authenticated user's profile, changing password, logging out, or preparing user contact fields for orders."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab me --help"
---

# Biolab Users

**Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

## Commands

```bash
biolab status
biolab me -f json
biolab me update '{"phone_number":"13800000000"}' -f json
biolab me change-password --current <OLD_PASSWORD> --new <NEW_PASSWORD>
biolab logout
```

## Schema

Inspect `<BIOLAB_BASE_URL>/openapi.json` before updating user fields.

Relevant schemas:

- `UserUpdateMe`: optional `full_name`, `email`, `phone_number`
- `UpdatePassword`: requires `current_password`, `new_password`

## Rules

- Do not print tokens, passwords, or secrets.
- Prefer `biolab status` before assuming token validity.
- Confirm before updating profile fields.
- Never ask the user to paste a password into chat unless the user explicitly chooses that flow.
- Use `biolab me -f json` to prefill order contact fields only after user confirmation.
- Do not invent admin user-management commands from backend `users` endpoints; this skill is scoped to the authenticated account unless the CLI exposes more commands.
