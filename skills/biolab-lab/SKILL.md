---
name: biolab-lab
version: 0.1.0
description: "Use when managing Biolab lab information, members, roles, invitations, join applications, approval settings, or lab approval rules."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab lab --help"
---

# Biolab Lab

**Before starting, read `../biolab-shared/SKILL.md` for auth, safety, and OpenAPI rules.**

## Commands

```bash
biolab lab info -f json
biolab lab create <NAME> -f json
biolab lab update '{"require_approval":true}' -f json
biolab lab members -f json
biolab lab update-role <USER_ID> member -f json
biolab lab remove-member <USER_ID> -f json
biolab lab invite <EMAIL> member -f json
biolab lab invitations -f json
biolab lab accept-invite <INVITATION_ID> -f json
biolab lab decline-invite <INVITATION_ID> -f json
biolab lab join <LAB_ID> member -f json
biolab lab applications -f json
biolab lab approve-app <APPLICATION_ID> -f json
biolab lab reject-app <APPLICATION_ID> -f json
biolab lab approval-rules -f json
biolab lab add-rule '{"order_type":"primer_synthesis","max_price":500,"approver_role":"pi"}' -f json
biolab lab remove-rule <RULE_ID> -f json
```

## Schema

Inspect `<BIOLAB_BASE_URL>/openapi.json` before preparing lab JSON or role changes.

Relevant schemas:

- `LabCreate`: requires `name`
- `LabUpdate`: optional `name`, `require_approval`
- `LabMemberUpdate`: requires `role`
- `LabInviteRequest`: requires `email`; optional `role`
- `LabJoinRequest`: optional `role`
- `LabApprovalRuleCreate`: optional `order_type`, `max_price`, `approver_role`

Roles are backend strings. Common project roles include `pi`, `procurement`, `finance`, `warehouse`, and `member`; verify current lab policy before assigning roles.

## Rules

- Confirm before removing members, changing roles, approving applications, rejecting applications, or editing approval rules.
- Use the lowest sufficient role.
- Read `lab members -f json` before changing a member role unless the user provides an exact `user_id`.
- Do not assume role hierarchy is enforced by the CLI.
