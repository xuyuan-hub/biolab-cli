# Users Reference

Use this reference before reading or updating the authenticated user's account.

## Recommended Commands

```bash
biolab me -f json
biolab me update '{"phone_number":"13800000000"}'
biolab me change-password --current <OLD_PASSWORD> --new <NEW_PASSWORD>
biolab status
biolab logout
```

## Agent Rules

- Do not print tokens, passwords, or secrets.
- Prefer `biolab status` before assuming the token is valid.
- Confirm before updating profile fields.
- Never ask the user to paste a password into chat unless the user explicitly chooses that flow; prefer interactive/manual handling for password changes.
