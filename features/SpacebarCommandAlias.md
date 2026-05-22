# Spacebar Command Alias

## Status

Implemented.

## Related Specifications

This feature extends the implemented [Bullet Journal TUI](./BulletJournalTUI.md)
MVP and the implemented [Fuzzy Command Pane Search](./FuzzyCommandPaneSearch.md)
feature. Those specs define pane focus, journal-pane keyboard handling, command
entry, fuzzy command search, and context-specific journal actions.

## Purpose & User Problem

When the journal pane is focused, entering command mode should be fast and
comfortable. The user currently presses `:` to open command entry from the
journal pane. This feature adds the spacebar as an alias for that same `:`
behavior while the journal pane is focused.

The alias is intended to reduce friction for keyboard-driven journaling without
changing existing command syntax or command-pane text entry.

## Success Criteria

- When the journal pane is focused, pressing `Space` behaves the same as
  pressing `:`.
- When the journal pane is focused, pressing `Space` opens the same command
  entry or fuzzy command search UI that `:` opens.
- Opening command entry via `Space` displays the same colon-prefixed prompt as
  opening it via `:`.
- When an entry is highlighted in the focused journal pane, pressing `Space`
  preserves the same context-specific command behavior as pressing `:`.
- Existing `:` behavior remains unchanged.
- Spacebar input continues to insert a literal space anywhere text entry is
  expected, including command entry, fuzzy command search text, and selected
  command text entry.
- Spacebar does not trigger command entry when the command pane is focused.
- Existing journal navigation, highlighting, command submission, and escape
  behavior continue to work.

## Scope

- Add `Space` as a keyboard alias for `:` only when the journal pane is focused.
- Reuse the existing `:` handling path instead of adding a separate command-mode
  implementation.
- Preserve command syntax such as `:n`, `:e`, `:f`, `:t`, `:x`, `:c`, and `:q`.
- Do not add or change status/help text for the alias.
- Cover the alias with tests where the existing input handling is testable.

## Constraints

- Preserve the Rust and ratatui application stack.
- Preserve the existing Markdown storage model.
- Do not change the command parser or persisted command syntax.
- Do not make `Space` a global shortcut outside the focused journal pane.
- Do not remove or remap the existing `:` shortcut.

## Technical Considerations

- The implementation should route journal-pane `Space` key handling through the
  same logic used by journal-pane `:` key handling.
- The focused command pane and any active text-entry mode must continue treating
  `Space` as normal character input.
- If input handling is centralized around key events, this should be a small
  conditional in the journal-pane key branch.
- Tests should cover at least:
  - `Space` from the focused journal pane enters the same mode as `:`.
  - `Space` in command/text entry remains literal text input.
  - Existing `:` behavior still passes.

## Out Of Scope

- Adding other command aliases.
- Changing command names or command syntax.
- Changing fuzzy matching behavior.
- Changing journal-pane navigation keys.
- Adding mouse interaction.
- Changing the default focused pane.

## Open Questions

None.
