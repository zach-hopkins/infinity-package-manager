## Change scope

Describe the real mod, installer behavior, or reproducibility problem this change addresses.
Read [LIVING_CONTEXT.md](../LIVING_CONTEXT.md) before completing this checklist.

## Foundation check

- [ ] Does this change preserve existing lockfile meaning or include an explicit migration?
- [ ] If it changes a schema, relationship, capability, or provenance concept, is there a documented real-world case and regression fixture?
- [ ] Does it preserve the separation of environment graph, artifact acquisition, materialization, and A5 execution?
- [ ] Does it avoid treating lineage as an automatic package substitute?
- [ ] Does it avoid recording local paths, secrets, or other non-portable state in a lockfile?
- [ ] If it affects execution, does `analysis-only` remain blocked before filesystem mutation?

## Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace --locked`
