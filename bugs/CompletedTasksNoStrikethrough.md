# Completed Tasks No Strikethrough

## Status

Implemented.

## Related Specifications

This bug refines the completed-task rendering behavior from the implemented
[Bullet Journal TUI](../features/BulletJournalTUI.md) MVP and the implemented
[Entry Specific Actions](../features/EntrySpecificActions.md) feature.

Those specs define task completion, task cancellation, journal-pane rendering,
and Markdown persistence. This bug supersedes only the prior expectation that
completed tasks should use strikethrough.

## Purpose & User Problem

Completed tasks are currently rendered as struck-through text. That makes
completion look visually similar to cancellation and makes completed task text
harder to read.

Users need completed tasks to remain readable while still being visibly marked
complete. The `X` task marker should carry completion state. Strikethrough
should be reserved for cancelled entries.

## Existing Behavior

- Completing a task changes its marker to `X`.
- Completed tasks are rendered with strikethrough in the TUI.
- Completed tasks are persisted to Markdown as `X ~~task text~~`.
- Cancelled tasks and cancelled events are also rendered with strikethrough.

## Expected Behavior

- Completing a task still changes its marker to `X`.
- Completed tasks are rendered without strikethrough in the TUI.
- Completed tasks are persisted to Markdown without strikethrough as
  `X task text`.
- Cancelled tasks and cancelled events continue to render with strikethrough in
  both the TUI and Markdown.
- Existing Markdown using the previous completed-task form,
  `X ~~task text~~`, should still parse as a completed task with text
  `task text`.
- When an existing completed task is saved again through the normal app save
  flow, Markdown should normalize to the new non-strikethrough completed-task
  form.

## Success Criteria

- A completed task is visibly marked with `X` and its text is not crossed out in
  the app.
- Saving a completed task writes `X task text`, not `X ~~task text~~`.
- Loading either `X task text` or legacy `X ~~task text~~` produces a completed
  task with clean text.
- Cancelled tasks still save as `· ~~task text~~`.
- Cancelled events still save as `◦ ~~event text~~`.
- Tests cover completed-task Markdown rendering, legacy completed-task parsing,
  and cancelled-entry strikethrough behavior.

## Scope

- Update completed-task rendering in the TUI.
- Update completed-task Markdown serialization.
- Preserve parsing compatibility for legacy completed-task Markdown.
- Update focused tests and any active user-facing command documentation that
  describes the completed-task Markdown format.

## Constraints

- Preserve the Rust and ratatui stack.
- Preserve the current Markdown file organization under `journal/YYYY-MM-DD.md`.
- Preserve existing task completion, reopening, cancellation, and persistence
  commands.
- Do not rewrite historical feature specs beyond linking this bug as the current
  correction unless explicitly requested.

## Technical Considerations

- The current domain model exposes a shared struck-through check for both
  completed and cancelled entries. Completed task display should be separated
  from cancelled-entry display.
- Markdown serialization should format completed tasks directly from their text
  instead of using the strikethrough wrapper.
- Markdown parsing should continue to unwrap legacy `~~` for completed task
  lines so old journal files do not show literal tildes after this fix.
- UI tests should assert render output or style behavior strongly enough to
  catch accidental reintroduction of crossed-out completed tasks.

## Out Of Scope

- Changing task completion command names or keyboard flow.
- Allowing completed tasks to be cancelled without reopening them first.
- Changing note, feeling, raw-entry, or date-switching behavior.
- Migrating every historical journal file in a batch operation.
- Adding a one-time migration command.
- Introducing a new storage backend or rich text format.

## Open Questions

None.
