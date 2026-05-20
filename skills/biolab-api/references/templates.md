# Templates Reference

Use this reference before creating or applying order-info templates.

## Core Concepts

Templates store recurring order metadata such as company, invoice title, PI, payment method, recipient address, and contact fields.

## Recommended Commands

```bash
biolab templates list -f json
biolab templates get <TEMPLATE_ID> -f json
biolab templates get-default primer_synthesis -f json
biolab templates create template.json
biolab templates update <TEMPLATE_ID> template.json
biolab templates set-default <TEMPLATE_ID>
```

## Agent Rules

- Prefer reading the default template before creating an order JSON.
- Confirm before deleting templates or changing the default template.
- Treat template data as defaults, not as guaranteed final order values; user-provided values win.

## Common Fields

```json
{
  "name": "Default primer order",
  "order_type": "primer_synthesis",
  "company_name": "Company",
  "invoice_title": "Invoice title",
  "principal_investigator": "PI",
  "payment_method": "bank_transfer",
  "recipient_address": "Address",
  "customer_name": "Name",
  "customer_phone": "13800000000",
  "customer_email": "name@example.com"
}
```
