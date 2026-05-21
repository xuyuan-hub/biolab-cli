---
name: biolab-inventory
version: 0.1.0
description: "Use when listing Biolab primer stock, checking inventory details or stats, moving stock in or out, managing storage locations, or reviewing inventory transactions."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab inventory --help"
---

# Biolab Inventory

**Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

## Commands

```bash
biolab inventory list -f json
biolab inventory list --primer-name <NAME> -f json
biolab inventory list --location-id <LOCATION_ID> -f json
biolab inventory list --low-stock -f json
biolab inventory get <STOCK_ID> -f json
biolab inventory stats -f json
biolab inventory checkin <STOCK_ID> --quantity 5 --purpose "restock" -f json
biolab inventory checkout <STOCK_ID> --quantity 2 --purpose "PCR" --experiment-ref "EXP-001" -f json
biolab inventory locations -f json
biolab inventory create-location "Box A" --parent-id <LOCATION_ID> -f json
```

## Schema

Inspect `<BIOLAB_BASE_URL>/openapi.json` before preparing inventory JSON or interpreting fields.

Relevant schemas:

- `CheckinRequest`: requires `quantity`; optional `purpose`
- `CheckoutRequest`: requires `quantity`; optional `purpose`, `experiment_ref`
- `StorageLocationCreate`: requires `name`; optional `parent_id`
- `StoragePreferenceCreate`: backend-only in the current CLI

## Rules

- Confirm before checkin or checkout because these mutate inventory.
- Quantity must be positive.
- Use `inventory get` before checkout when remaining quantity matters.
- Include `experiment_ref` for checkout whenever the user provides experiment context.
- Do not invent commands for backend endpoints not exposed by `biolab inventory --help`.
