# Vim Keyboard Navigation

## Status

Implemented.

## Related Specs

This enhancement builds on the keyboard-driven TUI flow defined in
[BulletJournalTUI.md](./BulletJournalTUI.md), the single-pane day navigation in
[SwitchJournalDay.md](./SwitchJournalDay.md), and the active split-pane behavior
in [SplitJournalDays.md](./SplitJournalDays.md).

## Summary

Add vim-style `h`, `j`, `k`, and `l` navigation aliases while the journal pane
is focused. These keys should behave exactly like `Left`, `Down`, `Up`, and
`Right`, respectively.

## Purpose & User Problem

Users who navigate with vim conventions should be able to move around the
journal pane without leaving the home row. This should preserve the existing
arrow-key behavior and make vim navigation an additional shortcut, not a
replacement.

## Success Criteria

- When the journal pane is focused, pressing `h` behaves the same as `Left`.
- When the journal pane is focused, pressing `j` behaves the same as `Down`.
- When the journal pane is focused, pressing `k` behaves the same as `Up`.
- When the journal pane is focused, pressing `l` behaves the same as `Right`.
- In single-journal view, `h` and `l` switch to the previous and next journal
  day with the same behavior as the arrow keys.
- In split-journal view, `h` and `l` focus or shift the active split pane with
  the same behavior as the arrow keys.
- In both single and split journal views, `j` and `k` move the highlighted entry
  within the active journal pane with the same behavior as the arrow keys.
- Vim navigation is always enabled and does not require configuration.
- Text input contexts continue to accept `h`, `j`, `k`, and `l` as typed
  characters instead of navigation commands.

## Scope & Constraints

- Scope is limited to adding journal-pane key aliases for existing navigation
  behavior.
- The aliases apply only when the app focus is `Focus::Journal`.
- Command search, command entry, and other text input modes must preserve normal
  character input for `h`, `j`, `k`, and `l`.
- Existing arrow-key behavior must remain unchanged.
- No new settings, toggles, or user configuration are introduced.
- No new navigation semantics are introduced beyond parity with the existing
  arrow keys.

## Technical Considerations

- Journal key handling currently lives in `src/app.rs` in the journal-focused
  key path.
- The implementation should reuse the same selection and navigation methods
  already used by `Up`, `Down`, `Left`, and `Right` rather than duplicating
  movement behavior.
- Key handling should only treat unmodified lowercase `h`, `j`, `k`, and `l`
  typed in the journal pane as navigation aliases.
- Tests should cover single-journal aliases, split-view active-pane aliases, and
  the fact that text input modes still receive these characters normally.

## Out of Scope

- Vim modes.
- Insert/normal mode state.
- Uppercase vim commands.
- Count prefixes such as `3j`.
- Word, line, page, or document movement commands.
- Configurable key bindings.
- Applying vim aliases outside the journal pane.

## Open Questions

None.
