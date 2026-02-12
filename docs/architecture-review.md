# Architecture and Module Organization Review (Actionable)

## Why this revision
The previous review was broad but not concrete enough to help you stop the refactor loop around `WslManager`.
This version gives a **single pragmatic target** with minimal churn.

## The key decision (to end the loop)
Treat `WslManager` as the **application orchestrator for WSL use-cases** (not just a thin engine wrapper, and not a pure domain object).

That means:
- `WslManager` owns operation flow (`validate environment`, `prepare cloud-init`, `delete/create instance`, dry-run branching).
- Adapters do side effects (CLI calls, filesystem, OS probing).
- Reporting/log formatting stays outside.

This is a stable, scalable middle ground for CLI apps.

---

## Concrete target responsibilities

### `src/wsl/manager.rs` should own
- Public use-case methods:
  - `validate_environment(...) -> EnvironmentReport`
  - `create_instance(...) -> CreateReport`
- Use-case sequencing and branching.
- Coordination of dependencies via traits/interfaces.

### `src/wsl/services/*` should become
- Either:
  1. Internal private helpers called by manager, or
  2. Folded into `manager.rs` if tiny.

If service files remain, they should not feel like a competing orchestrator layer.

### `src/wsl/engine/*`, `cloud_init/*`, `maintenance/*`, `validation/*` should be adapters/policies
- `engine/*`: WSL command execution adapters.
- `cloud_init/*`: template load/render/store adapters.
- `validation/config.rs`: pure config rules.
- `validation/environment.rs`: split later into:
  - policy (`what must be true`)
  - probes (`how to ask Windows/dism/wsl.exe`)

### `src/app.rs` should own only app bootstrap/routing
- Parse app-level flags/config and instantiate manager + dependencies.
- Call manager use-cases.
- Delegate output rendering to `reporting/*`.

---

## What feels misplaced today

1. `app::run` coordinates too much directly.
   - It currently reaches into environment checks, maintenance update, config loading, manager calls, and reporting.
   - This makes `app` a second orchestrator beside `WslManager`.

2. `WslManager` is underpowered.
   - It mainly forwards calls while real branching lives in services/free functions.
   - This creates ambiguity: “where should new behavior go?”

3. Environment checks and update are split awkwardly.
   - `validation::environment` and `maintenance::environment` are separate, but coordination sits in `app`.
   - Better to coordinate inside manager/use-case so app layer remains thin.

4. Reporting is currently tied to low-level structures.
   - Fine for now, but medium-term, manager returning stable DTO/report types keeps CLI output concerns isolated.

---

## Minimal refactor plan (no big rewrite)

### Step 1 (do this first)
Move orchestration currently in `app::run` into manager methods.

Concretely:
- Add `WslManager::apply_config(root_config, options) -> Vec<CreateReport>` (or a richer aggregate report).
- Keep `app::run` to: ensure OS, load config, call manager, pass results to reporting.

**Benefit:** one orchestrator, one place for branching decisions.

### Step 2
Decide fate of `CreateInstanceService`:
- If it stays: make it explicitly an internal helper to manager.
- If it goes: inline logic into manager and keep tiny helper functions.

**Rule:** contributors should never wonder “manager or service?”

### Step 3
Unify environment lifecycle entrypoints behind manager:
- `manager.validate_environment()`
- `manager.update_wsl_if_needed(dry_run)` (or combined `prepare_environment`)

**Benefit:** app no longer coordinates multiple environment modules.

### Step 4 (optional, later)
Split `validation/environment.rs` into policy + probe adapter traits for easier tests.

---

## “Definition of done” for architecture
You are done (and can stop refactoring) when:

- `app.rs` is thin bootstrap + routing.
- `WslManager` is the single orchestration entrypoint for WSL use-cases.
- Side effects are behind adapters (`engine`, `cloud_init`, Windows probes).
- Reporting only formats returned reports/events.
- New feature placement is obvious without debate.

---

## Guardrails to avoid circular refactors

- If a change introduces branching in `app.rs`, move it to manager.
- If a module both decides policy and executes OS commands, split those concerns when touched.
- If two files can both “own” a use-case, pick manager as the owner and demote the other to helper.

This gives a consistent placement rule and should stop the “round and round” feeling.
