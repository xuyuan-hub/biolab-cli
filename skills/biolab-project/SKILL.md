---
name: biolab-project
description: "Use when operating Biolab project-scoped seed workflows by slug, including Tashan germplasm records, sequencing files, germplasm stocks, planting orders, planting items, and harvest records."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab project --help"
---

# Biolab Project Workflows

**Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

Use this skill for project-scoped APIs under `/project/{slug}/...`. Use `biolab projects ...` for project administration by project id.

## Discover Slug

```bash
biolab projects list -f json
biolab project <SLUG> info -f json
```

Use the `slug` field from `projects list`, for example `tashan`.

## Germplasm

```bash
biolab project <SLUG> germplasm list -f json
biolab project <SLUG> germplasm list --search <TEXT> -f json
biolab project <SLUG> germplasm list --filters '[{"field":"name","operator":"contains","value":"A"}]' -f json
biolab project <SLUG> germplasm get <GERMPLASM_ID> -f json
biolab project <SLUG> germplasm create '<JSON>' -f json
biolab project <SLUG> germplasm update <GERMPLASM_ID> '<JSON>' -f json
biolab project <SLUG> germplasm delete <GERMPLASM_ID> -f json
biolab project <SLUG> germplasm sequencing-files <GERMPLASM_ID> -f json
biolab project <SLUG> germplasm stocks <GERMPLASM_ID> -f json
```

## Planting

```bash
biolab project <SLUG> planting list -f json
biolab project <SLUG> planting get <ORDER_ID> -f json
biolab project <SLUG> planting create '<JSON>' -f json
biolab project <SLUG> planting update <ORDER_ID> '<JSON>' -f json
biolab project <SLUG> planting items <ORDER_ID> -f json
biolab project <SLUG> planting harvests <ORDER_ID> -f json
biolab project <SLUG> planting create-harvest <ORDER_ID> '<JSON>' -f json
```

## Schema

Inspect `<BIOLAB_BASE_URL>/openapi.json` before preparing create/update JSON.

Relevant schemas:

- `GermplasmCreate`, `GermplasmUpdate`, `GermplasmResponse`
- `PlantingOrderCreate`, `PlantingOrderUpdate`, `PlantingOrderResponse`
- `PlantingOrderItemResponse`
- `HarvestCreateRequest`, `HarvestResponse`
- `StockResponse`

`germplasm list --filters` must be a JSON array string such as:

```json
[{"field":"name","operator":"contains","value":"A"}]
```

## Rules

- Confirm before creating, updating, or deleting germplasm records.
- Confirm before creating or updating planting orders or harvest records.
- Prefer `-f json` for all project workflows.
- Do not use project slug workflows for user administration, signup, password recovery, or audit/log endpoints.
