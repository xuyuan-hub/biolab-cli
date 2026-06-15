---
name: biolab-inventory
description: "Use when listing, checking, creating, adjusting, transferring, or consuming Biolab generic inventory: item definitions, stock batches, locations, stock transactions, experiment inventory checks, checkin, checkout, FIFO checkout by item, and task-linked inventory usage."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab inventory --help"
---

# Biolab Inventory

Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.

Use this skill for generic inventory, not only primer stock. The current CLI supports backend OpenAPI inventory endpoints that already exist; do not invent reservation, atomic lock, or purchase-order commands.

## Read Commands

```bash
biolab inventory items --search <NAME> --category <CATEGORY> --supplier <SUPPLIER> -f json
biolab inventory item <ITEM_ID> -f json
biolab inventory list --name <NAME> --location-id <LOCATION_ID> --low-stock -f json
biolab inventory list --primer-name <OLD_PRIMER_NAME> -f json
biolab inventory get <STOCK_ID> -f json
biolab inventory summary --search <NAME> --category <CATEGORY> -f json
biolab inventory transactions <STOCK_ID> -f json
biolab inventory transactions-all --type checkout --item-id <ITEM_ID> --search <NAME> -f json
biolab inventory locations -f json
biolab inventory preferences --workflow-type <TYPE> -f json
```

`--primer-name` is only a compatibility alias for old primer workflows. Prefer `--name` for generic inventory.

## Write Commands

Confirm with the user before mutating inventory unless the user explicitly asked to execute the mutation.

```bash
biolab inventory create-item item.json -f json
biolab inventory update-item <ITEM_ID> item_update.json -f json
biolab inventory disable-item <ITEM_ID> -f json
biolab inventory create-stock stock.json -f json
biolab inventory checkin <STOCK_ID> --quantity <QTY> --purpose "<WHY>" -f json
biolab inventory checkout <STOCK_ID> --quantity <QTY> --recipient "<WHO>" --purpose "<WHY>" --experiment-ref <EXP> --task-id <TASK_ID> --part-id <PART_ID> --requirement-key <KEY> -f json
biolab inventory checkout-item <ITEM_ID> --quantity <QTY> --purpose "<WHY>" --experiment-ref <EXP> --task-id <TASK_ID> --part-id <PART_ID> --requirement-key <KEY> -f json
biolab inventory adjust <STOCK_ID> --quantity -1 --type loss --reason "<REASON>" -f json
biolab inventory transfer <STOCK_ID> --location-id <LOCATION_ID> --reason "<REASON>" -f json
biolab inventory create-location "<NAME>" --parent-id <PARENT_ID> -f json
```

Rules:

- `checkin`, `checkout`, and `checkout-item` quantities must be positive.
- `adjust` quantity may be positive or negative, but not zero. Allowed types are `correction`, `loss`, `damage`, and `expire`.
- Prefer `checkout-item` when an experiment specifies the item but not the exact batch; the backend performs FIFO stock-out.
- Prefer `checkout` when a specific stock batch was selected or physically used.
- Always include `experiment_ref` when there is experiment context.
- Always include `task_id`, `part_id`, and `requirement_key` when inventory is consumed for a scheduled experiment task.

## Inventory Check

Use this before creating an executable experiment task or experiment plan:

```bash
biolab inventory check requirements.json -f json
```

Supported requirement file shape:

```json
{
  "requirements": [
    {
      "requirement_key": "pcr.dntp",
      "name": "dNTP Mix",
      "quantity": 10,
      "unit": "uL",
      "category": "reagent"
    }
  ]
}
```

The check command is a client-side aggregate query using current backend read APIs. It is not a reservation, atomic lock, or guarantee that the stock will still exist at execution time.

Interpretation:

- `available`: enough matching stock currently exists.
- `insufficient_stock`: the item exists but matching stock is too low.
- `missing_item`: no item definition was found.
- `ambiguous_item`: multiple exact item definitions matched; ask the user to select one.

If unit differs, do not convert automatically. Ask the user or use the backend/source record if a conversion is explicitly available.

## Experiment Rule

For planning: check inventory only. Do not checkout while merely drafting a plan.

For execution: re-check inventory immediately before physical execution, then run `checkout` or `checkout-item` for each consumed requirement and record the task/part/requirement fields.

If stock is missing or insufficient, stop before creating a task marked executable. Move the missing material into the ordering workflow if the available order APIs support that material type.
