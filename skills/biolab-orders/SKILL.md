---
name: biolab-orders
version: 0.1.0
description: "Use when creating, listing, inspecting, updating, downloading, uploading, approving, or placing Biolab primer synthesis or sequencing orders, including primer purchase workflows."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab orders --help"
---

# Biolab Orders

**Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

Use `../biolab-templates/SKILL.md` whenever an order should use saved order-info defaults.

## Commands

```bash
biolab orders list -f json
biolab orders get <ORDER_ID> -f json
biolab orders create-primer order.json -f json
biolab orders create-sequencing order.json -f json
biolab orders update <ORDER_ID> '{"status":"received"}' -f json
biolab orders download <ORDER_ID> order.xlsx
biolab orders download-primer-template primer_template.xlsx
biolab orders download-sequencing-template sequencing_template.xlsx
biolab orders upload-primer-excel primer.xlsx -f json
biolab orders upload-sequencing-excel sequencing.xlsx -f json
```

## Schema

Inspect `<BIOLAB_BASE_URL>/openapi.json` before writing order JSON.

Relevant schemas:

- `PrimerOrderCreate`
- `PrimerItemCreate`
- `SequencingOrderCreate`
- `SequencingItemCreate`
- `OrderUpdate`

`OrderType`: `primer_synthesis`, `sequencing`

`OrderStatus`: `draft`, `pending_approval`, `approved`, `pending`, `ordered`, `received`, `stored`

## Primer Purchase Workflow

1. Read the default `primer_synthesis` template through `biolab-templates`.
2. Confirm template fields with the user.
3. Confirm contact fields, usually from `biolab me -f json`.
4. Collect supplier and primer items.
5. Build order JSON from OpenAPI schema.
6. Show the final summary and ask for explicit confirmation.
7. Run `biolab orders create-primer order.json -f json`.

At the time of writing, `PrimerItemCreate` requires only:

```json
{
  "primer_name": "FWD",
  "sequence": "ATGC"
}
```

Optional item fields currently include `base_count`, `purification_method`, `nmoles`, `scale_od`, `tube_count`, `deliverable_form`, `five_modification`, `three_modification`, `double_modification`, `primer_type`, and `remarks`.

Preserve order-level `notes` from the confirmed template unless the user overrides them.

## Rules

- Confirm before creating an order, changing order status, resending email, submitting for approval, approving, rejecting, or placing an order.
- Prefer Excel upload/download commands when the user is working from supplier spreadsheets.
- Do not assume supplier-specific fields; check OpenAPI and user-provided files.
