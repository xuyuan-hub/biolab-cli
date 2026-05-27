---
name: biolab-tashan
description: "Use when operating the Tashan (他山) project workflows: germplasm records, sequencing files, germplasm stocks, planting orders, planting items, and harvest records."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab project tashan --help"
---

# Biolab Tashan Project Workflows

**Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

Use this skill for all Tashan (他山) project-scoped APIs under `biolab project tashan ...`. Other projects have their own skill packages.

## Info

```bash
biolab project tashan info -f json
```

## Germplasm

```bash
biolab project tashan germplasm list -f json
biolab project tashan germplasm list --search <TEXT> -f json
biolab project tashan germplasm list --filters '[{"field":"name","operator":"contains","value":"A"}]' -f json
biolab project tashan germplasm get <GERMPLASM_ID> -f json
biolab project tashan germplasm create '<JSON>' -f json
biolab project tashan germplasm update <GERMPLASM_ID> '<JSON>' -f json
biolab project tashan germplasm delete <GERMPLASM_ID> -f json
biolab project tashan germplasm sequencing-files <GERMPLASM_ID> -f json
biolab project tashan germplasm stocks <GERMPLASM_ID> -f json
```

## Planting

```bash
biolab project tashan planting list -f json
biolab project tashan planting get <ORDER_ID> -f json
biolab project tashan planting create '<JSON>' -f json
biolab project tashan planting update <ORDER_ID> '<JSON>' -f json
biolab project tashan planting items <ORDER_ID> -f json
biolab project tashan planting harvests <ORDER_ID> -f json
biolab project tashan planting create-harvest <ORDER_ID> '<JSON>' -f json
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
- Do not use this skill for user administration, signup, password recovery, or audit/log endpoints.
