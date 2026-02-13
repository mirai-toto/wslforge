# Over-engineering Review

This document captures architecture and implementation areas that appear more complex than needed for the current feature set, with concrete simplification options.

## 1) Premature engine abstraction (`CliEngine` + `ApiEngine` + `EngineKind` + dynamic dispatch)

### Why it feels over-engineered
- The application always builds `EngineKind::Cli` in `app::build_engine`, so `EngineKind::Api` is unused by runtime behavior.
- `ApiEngine` exists only as an unimplemented stub that always errors.
- `WslManager` stores `Box<dyn WslEngine>`, which adds trait-object indirection and forces object-safe design before there is a second real implementation.

### Simpler alternative
- Replace `WslEngine` trait + `EngineKind` with one concrete `CliEngine` field in `WslManager`.
- Delete `ApiEngine` until a real second backend is needed.
- If a second backend appears later, reintroduce abstraction then (likely with an enum first, trait later only if necessary).

### Why this is better
- Fewer moving parts and fewer files to follow.
- Lower cognitive load for contributors.
- Eliminates dead/stub paths that currently only add maintenance surface.

---

## 2) Event-model indirection is heavier than current needs

### Why it feels over-engineered
- `WslManager` creates detailed event vectors (`EnvironmentEvent`, `ProfileEvent`) and returns structured reports.
- Reporting then maps those events to strings/icons in a second layer.
- The system currently has only one consumer (log output), so domain-event modeling duplicates simple control flow into event enums and renderers.

### Simpler alternative
- Move user-facing logging closer to workflow execution in `WslManager` (or a tiny helper), returning only a compact result enum (`Created`, `AlreadyExists`, `Skipped`) and errors.
- Keep event structs only if another non-log consumer is introduced (JSON output, UI, telemetry).

### Why this is better
- Removes translation layers between orchestration and logging.
- Makes behavior easier to trace (what happens is logged where it happens).
- Reduces branching duplication across orchestration + reporter modules.

---

## 3) Cloud-init split across many tiny modules

### Why it feels over-engineered
- `cloud_init` logic is spread across `orchestrate`, `load`, `render`, `store`, plus re-exports in `mod.rs`.
- For a single linear flow (load -> render -> write -> optional debug copy), file hopping is high relative to logic size.

### Simpler alternative
- Consolidate cloud-init implementation into one module (`cloud_init.rs`) with private helper functions.
- Keep only one public entry point (`prepare_cloud_init`).

### Why this is better
- Same functionality with fewer indirection points.
- Easier onboarding and debugging.
- Better local reasoning about side effects and error handling.

---

## 4) Double YAML parse fallback in config loading

### Why it feels over-engineered
- `load_yaml` parses once as `RootConfig`, then reparses as `Profile` to support a second format.
- Error output combines two parser failures, which is technically rich but more complex than needed for a CLI config UX.

### Simpler alternative
- Support exactly one canonical config shape (`RootConfig`).
- Optionally provide a tiny explicit migration helper command (`--migrate-config`) if single-profile convenience matters.

### Why this is better
- Less parser branching and simpler failure modes.
- Easier docs and mental model (“one valid schema”).

---

## 5) Configuration model carries template concerns

### Why it feels over-engineered
- `Profile` derives `Serialize` and includes serialization-focused attributes mainly to feed template rendering.
- This couples config schema details to rendering internals.

### Simpler alternative
- Keep config purely deserialize-focused.
- Build a small dedicated render context struct for templates.

### Why this is better
- Clear separation of concerns.
- Template evolution won’t force config-model serialization decisions.

---

## 6) Environment checks could be direct in current scope

### Why it feels over-engineered
- Validation builds environment event vectors and separate reporting messages/icons.
- For a command-line utility with immediate execution, pass/fail checks are the core value.

### Simpler alternative
- Perform environment checks directly with concise logs and immediate error returns.
- Return `()` for validation and keep only minimal structured data where actionable.

### Why this is better
- Less ceremony for straightforward prerequisite checks.
- Fewer enums and mapping functions to maintain.

---

## Suggested simplification sequence (low-risk first)

1. Remove `ApiEngine` + `EngineKind` and use concrete `CliEngine` in `WslManager`.
2. Collapse `cloud_init/*` into a single module.
3. Reduce report/event model to direct logs plus small result enums.
4. Drop dual config shape parsing and keep one YAML schema.
5. Introduce dedicated render context instead of serializing `Profile` directly.

These changes preserve behavior while significantly lowering complexity and future maintenance burden.
