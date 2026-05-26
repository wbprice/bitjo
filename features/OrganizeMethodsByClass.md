# Organize Methods By Class

## Status

Implemented.

## Related Specifications

This refactor preserves the behavior defined by the implemented
[Bullet Journal TUI](./BulletJournalTUI.md) specification and later feature and
bug specifications. Those specs remain the source of truth for application
behavior, command handling, UI behavior, and Markdown persistence.

## Purpose & User Problem

The current implementation should be easier to read by grouping methods and
helpers around the type or responsibility they primarily belong to. A developer
working in the app should be able to navigate `app.rs`, `ui.rs`, and
`journal.rs` without scanning unrelated behavior interleaved with a given type's
methods.

## Success Criteria

- `src/app.rs` groups application methods by the application type or
  responsibility they support.
- `src/ui.rs` groups UI rendering methods by the UI type or widget area they
  support.
- `src/journal.rs` groups journal domain methods by the journal type or entry
  type they support.
- Application behavior is preserved.
- Public API surface is preserved.
- Existing dependencies are unchanged.
- The code remains formatted according to Rust conventions.

## Scope

- Reorder and regroup methods in `src/app.rs`, `src/ui.rs`, and
  `src/journal.rs` for readability.
- Move private helper functions within those files when doing so improves
  method locality.
- Keep changes focused on organization and readability.
- Allow `cargo fmt` formatting changes.

## Constraints

- Do not change application behavior.
- Do not change public APIs.
- Do not add, remove, or upgrade dependencies.
- Do not change storage format, command semantics, keyboard behavior, or visual
  UI behavior.
- Avoid unrelated refactors outside `src/app.rs`, `src/ui.rs`, and
  `src/journal.rs`.

## Technical Considerations

- Prefer grouping `impl` methods so constructors, state accessors, command
  handlers, mutation helpers, and persistence helpers are easy to find.
- Prefer grouping rendering helpers near the widget or screen area they render.
- Keep private helpers close to their primary callers where practical.
- Use existing tests or add narrow tests only if the refactor reveals missing
  coverage around behavior that could be accidentally changed.
- Verification should include formatting and the existing Rust test suite.

## Out Of Scope

- Changing command behavior.
- Changing persisted Markdown output.
- Changing journal parsing.
- Changing the TUI layout or styling.
- Renaming public types or public methods.
- Moving code into new modules.
- Adding new dependencies.

## Open Questions

None.
