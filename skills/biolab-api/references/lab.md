# Lab Reference

Use this reference before managing lab membership, invitations, join applications, or approval rules.

## Core Roles

`pi > procurement > finance > warehouse > member`

## Recommended Commands

```bash
biolab lab info -f json
biolab lab members -f json
biolab lab invite <email> member
biolab lab update-role <USER_ID> procurement
biolab lab invitations -f json
biolab lab applications -f json
biolab lab approval-rules -f json
```

## Agent Rules

- Confirm before removing members, changing roles, approving applications, rejecting applications, or editing approval rules.
- Use the lowest sufficient role when inviting or updating users.
- Read `lab members -f json` before changing a member role unless the user provides an exact `user_id`.

## Approval Rules

Approval rules are lab-level workflow configuration. Use `approval-rules` to inspect, `add-rule` to add JSON configuration, and `remove-rule` to delete a rule.
