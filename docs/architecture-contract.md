# WSL Manager Architecture Contract

This file is the **stability contract** used during reviews to avoid repeated “manager too big / manager too thin” oscillation.

## Decision
`WslManager` is the **use-case orchestrator** for WSL workflows.

This is intentional.

## What `WslManager` must own
- Workflow sequencing and branching:
  - environment preparation order,
  - create/override/delete control flow,
  - dry-run branching,
  - per-profile iteration and aggregation.
- Calling adapters/policies in the right order.
- Returning operation reports (`EnvironmentReport`, `CreateReport`).

## What `WslManager` must NOT own
- Raw OS command execution details (`wsl.exe`, `dism.exe`) beyond delegating to adapters.
- Cloud-init file/template low-level implementation details.
- Log formatting and terminal presentation.

## Review checklist (pass/fail)
A review should be considered **pass** when all are true:
1. `app.rs` stays bootstrap/routing oriented (load config, call manager, pass reports to reporting).
2. New branching decisions are introduced in manager methods, not in `app.rs`.
3. Side-effect mechanics stay in helper/adapters (`engine`, `cloud_init`, `maintenance`, `validation`).
4. Reports remain operation-focused and are not mixed with presentation-specific formatting.

## Anti-churn rule
Do **not** request refactoring manager into a thin pass-through unless one of these is demonstrated:
- clear duplicate orchestration exists in another top-level module, or
- manager started embedding low-level shell/fs mechanics directly.

Without one of those conditions, keeping orchestration in manager is the intended architecture.
