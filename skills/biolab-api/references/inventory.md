# Inventory Reference

Use this reference before checking stock, moving stock in or out, or changing storage locations.

## Core Concepts

- Stock represents an available primer/material quantity.
- Transactions record `checkin` and `checkout` movements.
- Storage locations may be hierarchical through `parent_id`.

## Recommended Commands

```bash
biolab inventory list -f json
biolab inventory list --low-stock -f json
biolab inventory get <STOCK_ID> -f json
biolab inventory stats -f json
biolab inventory checkin <STOCK_ID> --quantity 5 --purpose "restock"
biolab inventory checkout <STOCK_ID> --quantity 2 --purpose "PCR" --experiment-ref "EXP-001"
biolab inventory locations -f json
biolab inventory create-location "Box A" --parent-id <LOCATION_ID>
```

## Agent Rules

- Confirm user intent before checkin/checkout because these mutate inventory.
- Quantity must be a positive number.
- Use `inventory get` before checkout when the current remaining quantity matters.
- Include `experiment_ref` for checkout whenever the user provides experiment context.
