# Important Journal Entry Flag

## Status

Implemented.

## Related Specifications

This feature extends the implemented [Bullet Journal TUI](./BulletJournalTUI.md)
MVP and should fit with highlighted-entry behavior described in
[Entry Specific Actions](./EntrySpecificActions.md).

## Purpose & User Problem

Users need a way to mark individual journal entries as important so notable
items can stand out from ordinary entries without changing the entry's type,
state, ordering, or search behavior.

## Success Criteria

- Any journal entry type can be marked important:
  - notes
  - events
  - feelings
  - tasks
- A highlighted entry can toggle importance with `:i`.
- A highlighted entry can toggle importance with the alias `:important`.
- Toggling importance on adds an important marker to the entry.
- Toggling importance off removes the important marker from the entry.
- Important entries are visually rendered with `* ` before the normal entry
  marker, such as `* - take out the trash` or `* X do my taxes`.
- Non-important entries are visually rendered with two leading spaces before the
  normal entry marker, such as `  - take out the trash`, so important and
  non-important entries keep their entry symbols aligned.
- Important entries are persisted to Markdown with `* ` before the normal entry
  marker, such as `* - take out the trash`.
- Non-important entries are persisted to Markdown with two leading spaces before
  the normal entry marker, such as `  - take out the trash`, so persisted
  Markdown matches TUI entry alignment.
- Importance is independent of task completion and cancellation state.
- Importance is independent of event cancellation state.
- Adding or removing importance persists the journal file immediately.
- Important entries do not affect sorting, filtering, search, or navigation.

## Scope

- Add an important flag for individual journal entries.
- Support importance for every existing journal entry type.
- Add `:i` as the command for toggling importance on the highlighted entry.
- Add `:important` as an alias for `:i`.
- Render important entries by prefixing the existing entry rendering with `* `.
- Render non-important entries by prefixing the existing entry rendering with
  two spaces.
- Persist important entries by prefixing the existing Markdown representation
  with `* `.
- Persist non-important entries by prefixing the existing Markdown
  representation with two spaces.
- Parse persisted important entries from Markdown when loading journal files.
- Parse persisted non-important entries from Markdown when loading journal
  files.

## Constraints

- Preserve the existing Rust and ratatui application stack.
- Preserve the existing Markdown storage model.
- Keep the feature local-only.
- Preserve existing entry symbols and state markers after the important prefix.
- Preserve compatibility with legacy Markdown files whose non-important entries
  do not have the two-space prefix.
- Do not introduce sorting, filtering, search, or navigation behavior for
  important entries in this version.

## Technical Considerations

- The journal entry domain model should track importance independently from
  entry type and existing task/event state.
- The command handler should route `:i` and `:important` to the currently
  highlighted journal entry.
- The importance toggle should be reversible and should not mutate any
  non-highlighted entry.
- Markdown rendering should add `* ` before an important entry's existing
  rendered form:
  - important note: `* - <text>`
  - important event: `* ◦ <text>`
  - important feeling: `* = <text>`
  - important incomplete task: `* · <text>`
  - important completed task: `* X <text>`
- Markdown rendering should add two spaces before a non-important entry's
  existing rendered form:
  - non-important note: `  - <text>`
  - non-important event: `  ◦ <text>`
  - non-important feeling: `  = <text>`
  - non-important incomplete task: `  · <text>`
  - non-important completed task: `  X <text>`
- Markdown parsing should recognize the leading `* ` marker before parsing the
  existing entry symbol or state marker.
- Markdown parsing should recognize the leading two-space non-important marker
  before parsing the existing entry symbol or state marker.
- Markdown parsing should continue to accept legacy unprefixed normal entries.
- Existing completed/cancelled rendering rules should continue to apply after
  the important prefix.
- TUI rendering should add a two-character importance column before the existing
  entry symbol: `* ` for important entries and two spaces for non-important
  entries.
- Tests should cover toggling importance for each entry type, alias behavior,
  Markdown rendering, Markdown parsing, TUI alignment, and persistence after
  toggling.

## Out Of Scope

- Sorting important entries separately.
- Filtering to important entries.
- Searching by importance.
- Jumping between important entries.
- Bulk marking entries important.
- Changing the creation commands for new entries to mark them important at
  creation time.
- Adding additional visual styling beyond the `* ` prefix.

## Open Questions

None.
