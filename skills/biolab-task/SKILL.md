---
name: biolab-task
version: 0.1.0
description: "Use when the user asks in natural language to do, implement, run, arrange, schedule, or execute a Biolab task, including Chinese requests like '帮我做/实现/安排/执行 <任务>'. First checks available task types and creates a task only when a matching type and required inputs are clear."
metadata:
  requires:
    bins: ["biolab"]
  cliHelp: "biolab tasks --help"
---

# Biolab Task Natural-Language Workflow

Use this skill when the user asks to do a concrete lab/workflow task in natural language, for example:

- `帮我做一下样品质检`
- `帮我实现这个测序数据分析任务`
- `安排一个样品接收任务`
- `执行 batch-03 的序列比对`
- `看看有没有任务类型可以做这个`
- `create a task for sample QC`

Do not use this skill for generic coding requests unless the user clearly means a Biolab task in the task scheduling system.

Before API calls, read `../biolab-shared/SKILL.md`.

## Core Rule

Never assume the task type exists. Always check available task types first:

```bash
biolab tasks types -f json
```

Then decide:

1. If one task type clearly matches, summarize it and collect missing required inputs.
2. If multiple task types may match, show the best 2-3 candidates and ask the user to choose.
3. If no task type matches, say no suitable task type is currently available and do not create a task.

## Matching Heuristics

Compare the user request with each task type's:

- `display_name`
- `key`
- `description`
- `category`
- `input_schema`
- `output_schema`
- `documents`

Prefer enabled task types. Ignore disabled task types unless the user explicitly asks about unavailable options.

Do not inspect, infer, or report task type staff bindings. Staff assignment/binding details are not part of the user-facing task type contract; if an API response includes `assigned_staff` or similar internal fields, ignore them.

## Creating A Task

Use:

```bash
biolab tasks create <json_file>
```

The JSON payload should follow `TaskCreate`:

```json
{
  "title": "<short user-facing title>",
  "description": "<optional description>",
  "task_type_id": "<matched_task_type_id>",
  "input_data": {},
  "parts": [
    {
      "name": "<part name>",
      "input_data": {}
    }
  ]
}
```

Do not hardcode `lab_id`. The CLI will try to fill it from the current lab, or the user can provide `--lab-id`.

If `input_schema.required` exists, ensure required fields are present before creating the task. If values are missing, ask concise follow-up questions.

## Confirmation

Before creating a task, show a short preview:

- matched task type
- task title
- input data
- parts

Ask for confirmation if the task would start external work, notify staff, spend resources, or if the request is ambiguous.

For clearly requested, low-risk task creation with all inputs present, proceed after the preview according to the user's intent.

## Examples

### Clear match with missing inputs

User: `帮我做样品质检`

Workflow:

1. Run `biolab tasks types -f json`.
2. Match `样品质检`.
3. Inspect required fields such as `sample_ids`.
4. Ask for missing sample IDs.
5. Create the task after inputs are complete.

### No match

User: `帮我做质谱分析`

If no task type mentions mass spectrometry or related schema fields, answer:

`当前可用任务类型里没有找到能覆盖“质谱分析”的类型。`

Do not create a generic task unless the user explicitly asks to create a custom/manual task and the API supports it.

## Output

After task creation, report:

- task id
- title
- status
- task type

Use `biolab tasks get <TASK_ID> -f json` only if the create response is missing important fields.
