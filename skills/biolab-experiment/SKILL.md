---
name: biolab-experiment
description: "Use when planning, creating, arranging, or executing Biolab experiments where reagents, consumables, primers, samples, or other inventory may be required. Enforces inventory checks before planning/tasks, ordering flow for missing stock, and task-linked checkout during execution."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab inventory --help; biolab tasks --help"
---

# Biolab Experiment

Before API calls, read:

- `../biolab-shared/SKILL.md`
- `../biolab-inventory/SKILL.md`
- `../biolab-task/SKILL.md` when creating or executing tasks

## Core Workflow

1. Extract inventory requirements from the experiment plan. Give every requirement a stable `requirement_key`, such as `pcr.polymerase`, `pcr.dntp`, or `sequencing.primer_f`.
2. Check inventory before calling the experiment executable or task-ready:

```bash
biolab inventory check requirements.json -f json
```

3. If every requirement is `available`, continue creating the plan or task.
4. If any requirement is `missing_item`, `ambiguous_item`, or `insufficient_stock`, stop and report what is missing. Do not claim the experiment is executable.
5. For missing primer stock, use the primer order workflow when appropriate. For generic reagents/consumables, tell the user the current CLI does not create generic purchase orders unless such an order endpoint exists.
6. During execution, re-check inventory and then checkout the actual consumed stock.

## Planning vs Execution

Planning phase:

- Check inventory.
- Record item candidates, stock candidates, quantities, units, and missing items in the plan.
- Do not mutate inventory.
- Do not treat `inventory check` as reservation.

Execution phase:

- Re-run `inventory check` as close as possible to physical execution.
- Use `checkout-item` when the item is known but no batch was chosen.
- Use `checkout` when a specific stock batch was selected.
- Include `experiment_ref`, `task_id`, `part_id`, and `requirement_key` on checkout whenever available.

Example:

```bash
biolab inventory checkout-item <ITEM_ID> \
  --quantity 10 \
  --purpose "PCR setup" \
  --experiment-ref EXP-20260615-001 \
  --task-id <TASK_ID> \
  --part-id <PART_ID> \
  --requirement-key pcr.dntp \
  -f json
```

## Requirement File

Use this shape for `inventory check`:

```json
{
  "requirements": [
    {
      "requirement_key": "pcr.polymerase",
      "name": "Phusion High-Fidelity DNA Polymerase",
      "quantity": 1,
      "unit": "uL",
      "category": "reagent"
    },
    {
      "requirement_key": "pcr.forward_primer",
      "name": "Primer F",
      "quantity": 1,
      "unit": "tube",
      "category": "primer"
    }
  ]
}
```

`requirement_key` must remain stable from plan creation to execution checkout.

## Ordering Rule

If stock is unavailable:

- Primer synthesis: use the Biolab orders skill/commands when the sequence and order information are available.
- Sequencing services: use sequencing order flow when the experiment requires sequencing service, not inventory checkout.
- Generic reagent or consumable: report the missing stock and ask the user how to order or restock unless a generic purchase endpoint exists in OpenAPI.

Never fake an order, reservation, or stock lock in the task payload.

## Task Integration

When creating experiment tasks:

- Include the inventory requirements in `input_data`.
- Include the inventory check report in `input_data` or an attached plan when useful.
- Do not mark the task ready for execution if required inventory is unavailable.

When completing execution:

- Checkout consumed inventory first.
- Include checkout transaction IDs in the task result or experiment execution record.
- If checkout fails due to insufficient stock, stop execution and report the failing requirement.
