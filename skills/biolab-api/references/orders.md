# Orders Reference

Use this reference before creating, updating, downloading, or resending Biolab orders.

## Core Concepts

- `primer_synthesis`: primer synthesis order.
- `sequencing`: sequencing order.
- Typical status flow: `pending -> ordered -> received -> stored`.
- Suppliers currently mentioned by project docs: `sangon`, `biosune`.

## Recommended Commands

```bash
biolab orders list -f json
biolab orders get <ORDER_ID> -f json
biolab orders create-primer order.json
biolab orders create-sequencing order.json
biolab orders update <ORDER_ID> '{"status":"received"}'
biolab orders download <ORDER_ID> order.xlsx
```

## Agent Rules

- Inspect an existing order or a template before writing a create JSON file.
- Prefer `-f json` when reading order details for follow-up automation.
- Confirm user intent before updating order status, resending mail, or creating an order.
- For Excel workflows, use download/upload template commands instead of inventing spreadsheet columns.

## Minimal Primer Order Shape

```json
{
  "type": "primer_synthesis",
  "supplier_name": "sangon",
  "customer_name": "Name",
  "customer_phone": "13800000000",
  "customer_email": "name@example.com",
  "items": [
    {
      "primer_name": "FWD",
      "sequence": "ATGC",
      "purification_method": "PAGE",
      "nmoles": 25
    }
  ]
}
```
